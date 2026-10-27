//! Per-key token-bucket rate limiting ().

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// Capacity (burst) and steady-state refill rate of a bucket.
#[derive(Clone, Copy, Debug)]
pub struct RateLimitConfig {
    /// Maximum tokens a bucket holds (the burst size).
    pub capacity: f64,
    /// Tokens added per second.
    pub refill_per_second: f64,
}

impl RateLimitConfig {
    /// A bucket that refills at `refill_per_second` and starts full at `burst`.
    pub fn new(refill_per_second: f64, burst: u32) -> Self {
        RateLimitConfig {
            capacity: burst.max(1) as f64,
            refill_per_second: refill_per_second.max(0.0),
        }
    }
}

struct Bucket {
    tokens: f64,
    updated: Instant,
}

/// A token-bucket limiter with one bucket per key (token or client address).
pub struct RateLimiter {
    config: RateLimitConfig,
    buckets: Mutex<HashMap<String, Bucket>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        RateLimiter {
            config,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Consumes one token for `key`. `Ok(())` when allowed; `Err(retry_after)`
    /// carries the whole seconds to wait before `Retry-After` expires.
    pub fn check(&self, key: &str) -> Result<(), u64> {
        let now = Instant::now();
        let mut buckets = self.buckets.lock().expect("rate limiter lock");
        self.prune(&mut buckets, now);

        let config = self.config;
        let bucket = buckets.entry(key.to_owned()).or_insert_with(|| Bucket {
            tokens: config.capacity,
            updated: now,
        });
        let elapsed = now.saturating_duration_since(bucket.updated).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * config.refill_per_second).min(config.capacity);
        bucket.updated = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            return Ok(());
        }

        let missing = 1.0 - bucket.tokens;
        let seconds = if config.refill_per_second > 0.0 {
            (missing / config.refill_per_second).ceil() as u64
        } else {
            u64::MAX
        };
        Err(seconds.max(1))
    }

    /// Drops buckets that have fully refilled, so idle keys do not accumulate.
    fn prune(&self, buckets: &mut HashMap<String, Bucket>, now: Instant) {
        const MAX_BUCKETS: usize = 4096;
        if buckets.len() <= MAX_BUCKETS {
            return;
        }
        let config = self.config;
        buckets.retain(|_, bucket| {
            let elapsed = now.saturating_duration_since(bucket.updated).as_secs_f64();
            bucket.tokens + elapsed * config.refill_per_second < config.capacity
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn allows_up_to_the_burst_then_rejects() {
        let limiter = RateLimiter::new(RateLimitConfig::new(1.0, 2));
        assert!(limiter.check("k").is_ok());
        assert!(limiter.check("k").is_ok());
        let retry = limiter.check("k").unwrap_err();
        assert!(retry >= 1, "retry-after should be at least one second");
    }

    #[test]
    fn keys_have_independent_buckets() {
        let limiter = RateLimiter::new(RateLimitConfig::new(0.0, 1));
        assert!(limiter.check("a").is_ok());
        assert!(limiter.check("b").is_ok());
        assert!(limiter.check("a").is_err());
    }

    #[test]
    fn refills_over_time() {
        let limiter = RateLimiter::new(RateLimitConfig::new(100.0, 1));
        assert!(limiter.check("k").is_ok());
        assert!(limiter.check("k").is_err());
        std::thread::sleep(Duration::from_millis(50));
        assert!(limiter.check("k").is_ok());
    }

    #[test]
    fn zero_refill_never_recovers() {
        let limiter = RateLimiter::new(RateLimitConfig::new(0.0, 1));
        assert!(limiter.check("k").is_ok());
        assert_eq!(limiter.check("k").unwrap_err(), u64::MAX);
    }
}
