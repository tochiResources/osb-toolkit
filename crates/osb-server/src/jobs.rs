//! A bounded job queue with workers, TTL and cancellation ().
//!
//! Long operations (builds) run off the request thread. The queue is bounded:
//! `submit` fails fast with [`QueueError::Full`] once the channel is saturated,
//! which the HTTP layer surfaces as `503` backpressure.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::package::InstallerConfig;
use crate::service::{Artifact, BuildRequest};

/// A job identifier.
pub type JobId = String;

/// The lifecycle state of a job.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded { artifact: Artifact },
    Failed { error: String },
    Cancelled,
}

impl JobStatus {
    fn is_finished(&self) -> bool {
        matches!(
            self,
            JobStatus::Succeeded { .. } | JobStatus::Failed { .. } | JobStatus::Cancelled
        )
    }
}

/// A job's public record.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobRecord {
    pub id: JobId,
    pub kind: String,
    pub status: JobStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

impl JobRecord {
    fn touch(&mut self) {
        self.updated_at = now_secs();
    }
}

/// The work a job performs.
#[derive(Clone, Debug)]
pub enum JobSpec {
    Build(BuildRequest),
    Package(Box<InstallerConfig>),
}

impl JobSpec {
    pub fn kind(&self) -> &'static str {
        match self {
            JobSpec::Build(_) => "build",
            JobSpec::Package(_) => "package",
        }
    }
}

/// Executes a job's work. Implemented by the service layer and by test stubs.
pub trait JobRunner: Send + Sync + 'static {
    fn run(&self, spec: JobSpec, cancel: &AtomicBool) -> Result<Artifact, String>;
}

/// Queue sizing and retention.
#[derive(Clone, Debug)]
pub struct QueueConfig {
    /// Concurrent workers (and thus concurrent builds).
    pub workers: usize,
    /// How many jobs may wait in the queue before `submit` returns `Full`.
    pub capacity: usize,
    /// How long a finished job is retained before it is reaped.
    pub job_ttl: Duration,
    /// Upper bound on tracked job records.
    pub max_tracked: usize,
}

impl Default for QueueConfig {
    fn default() -> Self {
        QueueConfig {
            workers: 2,
            capacity: 8,
            job_ttl: Duration::from_secs(300),
            max_tracked: 1024,
        }
    }
}

/// Why a submission was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    /// The queue is at capacity; the caller should retry later.
    Full,
}

struct Slot {
    record: JobRecord,
    cancel: Arc<AtomicBool>,
}

struct Inner {
    tx: mpsc::Sender<(JobId, JobSpec)>,
    jobs: Mutex<HashMap<JobId, Slot>>,
    next: AtomicU64,
    ttl: Duration,
    max_tracked: usize,
}

impl Inner {
    /// Drops the oldest finished jobs to stay within `max_tracked`.
    fn enforce_capacity(&self, jobs: &mut HashMap<JobId, Slot>) {
        if jobs.len() < self.max_tracked {
            return;
        }
        let mut finished: Vec<(JobId, u64)> = jobs
            .iter()
            .filter(|(_, slot)| slot.record.status.is_finished())
            .map(|(id, slot)| (id.clone(), slot.record.updated_at))
            .collect();
        finished.sort_by_key(|(_, updated)| *updated);
        let excess = jobs.len() + 1 - self.max_tracked;
        for (id, _) in finished.into_iter().take(excess) {
            jobs.remove(&id);
        }
    }
}

/// A bounded, worker-backed job queue.
#[derive(Clone)]
pub struct JobQueue {
    inner: Arc<Inner>,
}

impl JobQueue {
    /// Starts the queue. Must be called inside a Tokio runtime.
    pub fn start(config: QueueConfig, runner: Arc<dyn JobRunner>) -> JobQueue {
        let capacity = config.capacity.max(1);
        let (tx, rx) = mpsc::channel(capacity);
        let inner = Arc::new(Inner {
            tx,
            jobs: Mutex::new(HashMap::new()),
            next: AtomicU64::new(0),
            ttl: config.job_ttl,
            max_tracked: config.max_tracked.max(capacity + config.workers.max(1) + 1),
        });

        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        for _ in 0..config.workers.max(1) {
            let inner = Arc::clone(&inner);
            let rx = Arc::clone(&rx);
            let runner = Arc::clone(&runner);
            tokio::spawn(async move { worker(inner, rx, runner).await });
        }
        let reaper = Arc::clone(&inner);
        tokio::spawn(async move { reap_loop(reaper).await });

        JobQueue { inner }
    }

    /// Enqueues a job, returning `Full` when the queue is saturated.
    pub fn submit(&self, spec: JobSpec) -> Result<JobId, QueueError> {
        let id = new_id(self.inner.next.fetch_add(1, Ordering::Relaxed) + 1);
        let now = now_secs();
        {
            let mut jobs = self.inner.jobs.lock().expect("jobs lock");
            self.inner.enforce_capacity(&mut jobs);
            jobs.insert(
                id.clone(),
                Slot {
                    record: JobRecord {
                        id: id.clone(),
                        kind: spec.kind().to_string(),
                        status: JobStatus::Queued,
                        created_at: now,
                        updated_at: now,
                    },
                    cancel: Arc::new(AtomicBool::new(false)),
                },
            );
        }

        match self.inner.tx.try_send((id.clone(), spec)) {
            Ok(()) => Ok(id),
            Err(_) => {
                self.inner.jobs.lock().expect("jobs lock").remove(&id);
                Err(QueueError::Full)
            }
        }
    }

    /// The current record for `id`.
    pub fn status(&self, id: &str) -> Option<JobRecord> {
        let jobs = self.inner.jobs.lock().expect("jobs lock");
        jobs.get(id).map(|slot| slot.record.clone())
    }

    /// The artifact of a succeeded job, if any.
    pub fn artifact(&self, id: &str) -> Option<Artifact> {
        let jobs = self.inner.jobs.lock().expect("jobs lock");
        match &jobs.get(id)?.record.status {
            JobStatus::Succeeded { artifact } => Some(artifact.clone()),
            _ => None,
        }
    }

    /// Requests cancellation. `None` if unknown; `Some(false)` if already done.
    pub fn cancel(&self, id: &str) -> Option<bool> {
        let mut jobs = self.inner.jobs.lock().expect("jobs lock");
        let slot = jobs.get_mut(id)?;
        if slot.record.status.is_finished() {
            return Some(false);
        }
        slot.cancel.store(true, Ordering::Relaxed);
        if matches!(slot.record.status, JobStatus::Queued) {
            slot.record.status = JobStatus::Cancelled;
            slot.record.touch();
        }
        Some(true)
    }
}

async fn worker(
    inner: Arc<Inner>,
    rx: Arc<tokio::sync::Mutex<mpsc::Receiver<(JobId, JobSpec)>>>,
    runner: Arc<dyn JobRunner>,
) {
    loop {
        let received = {
            let mut guard = rx.lock().await;
            guard.recv().await
        };
        let Some((id, spec)) = received else {
            break;
        };

        let cancel = {
            let mut jobs = inner.jobs.lock().expect("jobs lock");
            let Some(slot) = jobs.get_mut(&id) else {
                continue;
            };
            if slot.cancel.load(Ordering::Relaxed) {
                slot.record.status = JobStatus::Cancelled;
                slot.record.touch();
                continue;
            }
            slot.record.status = JobStatus::Running;
            slot.record.touch();
            Arc::clone(&slot.cancel)
        };

        let runner = Arc::clone(&runner);
        let result = tokio::task::spawn_blocking(move || runner.run(spec, &cancel)).await;

        let mut jobs = inner.jobs.lock().expect("jobs lock");
        if let Some(slot) = jobs.get_mut(&id) {
            slot.record.status = if slot.cancel.load(Ordering::Relaxed) {
                JobStatus::Cancelled
            } else {
                match result {
                    Ok(Ok(mut artifact)) => {
                        artifact.id = id.clone();
                        JobStatus::Succeeded { artifact }
                    }
                    Ok(Err(error)) => JobStatus::Failed { error },
                    Err(join) => JobStatus::Failed {
                        error: join.to_string(),
                    },
                }
            };
            slot.record.touch();
        }
    }
}

async fn reap_loop(inner: Arc<Inner>) {
    let period = Duration::from_secs(1)
        .min(inner.ttl)
        .max(Duration::from_millis(50));
    let mut interval = tokio::time::interval(period);
    loop {
        interval.tick().await;
        let now = now_secs();
        let ttl = inner.ttl.as_secs();
        let mut jobs = inner.jobs.lock().expect("jobs lock");
        jobs.retain(|_, slot| {
            !(slot.record.status.is_finished() && now.saturating_sub(slot.record.updated_at) >= ttl)
        });
    }
}

fn new_id(sequence: u64) -> JobId {
    format!("job{sequence}-{:x}", nanos())
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::ArtifactKind;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Condvar;

    #[derive(Default)]
    struct StubRunner {
        gate: Option<Arc<(Mutex<bool>, Condvar)>>,
        calls: AtomicUsize,
    }

    impl JobRunner for StubRunner {
        fn run(&self, _spec: JobSpec, _cancel: &AtomicBool) -> Result<Artifact, String> {
            if let Some(gate) = &self.gate {
                let (lock, condvar) = &**gate;
                let mut released = lock.lock().unwrap();
                while !*released {
                    released = condvar.wait(released).unwrap();
                }
            }
            let call = self.calls.fetch_add(1, Ordering::Relaxed);
            Ok(Artifact {
                id: String::new(),
                name: format!("artifact-{call}"),
                target: "after-effects".into(),
                kind: ArtifactKind::File,
                path: "artifact.aex".into(),
                size: 4,
            })
        }
    }

    fn build_spec() -> JobSpec {
        JobSpec::Build(BuildRequest {
            crate_dir: ".".into(),
            target: "after-effects".into(),
            artifact_name: None,
            out_dir: None,
            extra_args: vec![],
        })
    }

    async fn wait_until_finished(queue: &JobQueue, id: &str) -> JobRecord {
        for _ in 0..500 {
            let record = queue.status(id).expect("job record");
            if record.status.is_finished() {
                return record;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("job did not finish");
    }

    async fn wait_for_running(queue: &JobQueue, id: &str) {
        for _ in 0..500 {
            if matches!(queue.status(id).unwrap().status, JobStatus::Running) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        panic!("job did not start running");
    }

    #[tokio::test]
    async fn runs_a_job_and_exposes_its_artifact() {
        let queue = JobQueue::start(QueueConfig::default(), Arc::new(StubRunner::default()));
        let id = queue.submit(build_spec()).unwrap();
        let record = wait_until_finished(&queue, &id).await;
        assert!(matches!(record.status, JobStatus::Succeeded { .. }));
        assert_eq!(record.kind, "build");

        let artifact = queue.artifact(&id).expect("artifact");
        assert_eq!(artifact.id, id);
        assert_eq!(artifact.name, "artifact-0");
    }

    #[tokio::test]
    async fn reports_backpressure_when_the_queue_is_full() {
        let gate = Arc::new((Mutex::new(false), Condvar::new()));
        let runner = Arc::new(StubRunner {
            gate: Some(Arc::clone(&gate)),
            calls: AtomicUsize::new(0),
        });
        let config = QueueConfig {
            workers: 1,
            capacity: 1,
            ..QueueConfig::default()
        };
        let queue = JobQueue::start(config, runner);

        let first = queue.submit(build_spec()).unwrap();
        wait_for_running(&queue, &first).await;
        let _second = queue.submit(build_spec()).unwrap();
        assert_eq!(queue.submit(build_spec()), Err(QueueError::Full));

        let (lock, condvar) = &*gate;
        *lock.lock().unwrap() = true;
        condvar.notify_all();
        wait_until_finished(&queue, &first).await;
    }

    #[tokio::test]
    async fn cancels_a_queued_job() {
        let gate = Arc::new((Mutex::new(false), Condvar::new()));
        let runner = Arc::new(StubRunner {
            gate: Some(Arc::clone(&gate)),
            calls: AtomicUsize::new(0),
        });
        let config = QueueConfig {
            workers: 1,
            capacity: 4,
            ..QueueConfig::default()
        };
        let queue = JobQueue::start(config, runner);

        let running = queue.submit(build_spec()).unwrap();
        let queued = queue.submit(build_spec()).unwrap();
        assert_eq!(queue.cancel(&queued), Some(true));
        assert!(matches!(
            queue.status(&queued).unwrap().status,
            JobStatus::Cancelled
        ));

        let (lock, condvar) = &*gate;
        *lock.lock().unwrap() = true;
        condvar.notify_all();
        wait_until_finished(&queue, &running).await;
    }

    #[test]
    fn unknown_jobs_are_absent() {
        let queue = JobQueue {
            inner: Arc::new(Inner {
                tx: mpsc::channel(1).0,
                jobs: Mutex::new(HashMap::new()),
                next: AtomicU64::new(0),
                ttl: Duration::from_secs(1),
                max_tracked: 4,
            }),
        };
        assert!(queue.status("missing").is_none());
        assert!(queue.artifact("missing").is_none());
        assert_eq!(queue.cancel("missing"), None);
    }
}
