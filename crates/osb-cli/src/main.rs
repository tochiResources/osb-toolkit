//! osb CLI (): `convert` / `build` / `bake3d` / `serve` over
//! the shared service layer, so tools (e.g. AeToOsb) can shell out without HTTP.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use clap::{Parser, Subcommand};
use osb_server::{
    AuthConfig, BuildRequest, ConvertRequest, QueueConfig, RateLimitConfig, ServerConfig, Service,
    ServiceConfig,
};

#[derive(Parser)]
#[command(
    name = "osb",
    version,
    about = "osb: convert effect scripts, build plug-ins and bake 3D scenes"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Converts a C# effect script into a plug-in crate.
    Convert {
        script: PathBuf,
        #[arg(short, long, default_value = "after-effects")]
        target: String,
        #[arg(short, long, default_value = "dist")]
        out: PathBuf,
        /// Extra converter flags (e.g. `--bundle`), after `--`.
        #[arg(last = true)]
        extra: Vec<String>,
    },
    /// Builds a generated crate into a host artifact (`.aex` / `.ofx.bundle`).
    Build {
        crate_dir: PathBuf,
        #[arg(short, long, default_value = "after-effects")]
        target: String,
        /// Where to write the artifact (defaults to the artifact root).
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Artifact stem (defaults to the crate directory name).
        #[arg(long)]
        name: Option<String>,
        /// Extra flags passed to `cargo build`, after `--`.
        #[arg(last = true)]
        extra: Vec<String>,
    },
    /// Bakes a plain 3D input JSON into command-span JSON.
    Bake3d {
        input: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Bakes a `{ worlds2d, worlds3d }` scene JSON into a storyboard JSON.
    Scene {
        input: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Starts the headless HTTP server. For CI/devops and headless use;
    /// the desktop app and `osbtk build` call the service layer directly.
    Serve {
        #[arg(short, long, default_value_t = 8787)]
        port: u16,
        /// Bootstrap admin token (falls back to `OSBTK_ADMIN_TOKEN`); a random one
        /// is generated and printed when neither is set.
        #[arg(long)]
        admin_token: Option<String>,
        /// Auth store path (falls back to `OSBTK_AUTH_FILE`, then the user config dir).
        #[arg(long)]
        auth_file: Option<PathBuf>,
        /// Disable the auth pipeline (open server; for trusted local use only).
        #[arg(long)]
        no_auth: bool,
        /// Access-token lifetime, in seconds.
        #[arg(long, default_value_t = 3600)]
        access_ttl: u64,
        /// Refresh-token lifetime, in seconds.
        #[arg(long, default_value_t = 2592000)]
        refresh_ttl: u64,
        /// Requests per second per key; `0` disables rate limiting.
        #[arg(long, default_value_t = 0.0)]
        rate_limit: f64,
        /// Burst size for the rate limiter.
        #[arg(long, default_value_t = 20)]
        rate_burst: u32,
        /// Concurrent build workers.
        #[arg(long, default_value_t = 2)]
        workers: usize,
        /// Queued jobs before the server applies backpressure.
        #[arg(long, default_value_t = 8)]
        queue_capacity: usize,
        /// How long a finished job is retained, in seconds.
        #[arg(long, default_value_t = 300)]
        job_ttl: u64,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Convert {
            script,
            target,
            out,
            extra,
        } => {
            let service = Service::detect()?;
            let result = service.convert(&ConvertRequest {
                script: osb_core::io::read_string(&script)?,
                file_name: script
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned()),
                target,
                extra_args: extra,
            })?;
            for file in &result.files {
                write_file(&out, &file.path, &file.contents)?;
            }
            eprintln!(
                "converted {} file(s) to {}",
                result.files.len(),
                out.display()
            );
            for diagnostic in &result.diagnostics {
                eprintln!("  {diagnostic}");
            }
        }
        Command::Build {
            crate_dir,
            target,
            out,
            name,
            extra,
        } => {
            let service = Service::with_config(ServiceConfig::in_process());
            let artifact = service.build(
                &BuildRequest {
                    crate_dir: crate_dir.to_string_lossy().into_owned(),
                    target,
                    artifact_name: name,
                    out_dir: out.map(|path| path.to_string_lossy().into_owned()),
                    extra_args: extra,
                },
                &AtomicBool::new(false),
            )?;
            eprintln!(
                "built {} ({}, {} bytes) -> {}",
                artifact.name, artifact.target, artifact.size, artifact.path
            );
        }
        Command::Bake3d { input, out } => {
            // The bake is in-process, so no converter CLI is required.
            let service = Service::with_config(ServiceConfig::in_process());
            let baked = service.bake3d(&osb_core::io::read_string(&input)?);
            match out {
                Some(path) => osb_core::io::write(&path, baked)?,
                None => println!("{baked}"),
            }
        }
        Command::Scene { input, out } => {
            // The bake is in-process, so no converter CLI is required.
            let service = Service::with_config(ServiceConfig::in_process());
            let baked = service.bake_scene(&osb_core::io::read_string(&input)?);
            match out {
                Some(path) => osb_core::io::write(&path, baked)?,
                None => println!("{baked}"),
            }
        }
        Command::Serve {
            port,
            admin_token,
            auth_file,
            no_auth,
            access_ttl,
            refresh_ttl,
            rate_limit,
            rate_burst,
            workers,
            queue_capacity,
            job_ttl,
        } => {
            let service = Service::detect()?;
            let auth = if no_auth {
                None
            } else {
                let admin_token = admin_token.or_else(|| std::env::var("OSBTK_ADMIN_TOKEN").ok());
                let file = auth_file
                    .or_else(|| std::env::var("OSBTK_AUTH_FILE").ok().map(PathBuf::from))
                    .or_else(osb_server::default_auth_path);
                Some(AuthConfig {
                    file,
                    admin_token,
                    access_ttl: Duration::from_secs(access_ttl),
                    refresh_ttl: Duration::from_secs(refresh_ttl),
                })
            };
            let config = ServerConfig {
                bind: SocketAddr::from(([127, 0, 0, 1], port)),
                auth,
                rate_limit: (rate_limit > 0.0)
                    .then(|| RateLimitConfig::new(rate_limit, rate_burst)),
                queue: QueueConfig {
                    workers,
                    capacity: queue_capacity,
                    job_ttl: Duration::from_secs(job_ttl),
                    ..QueueConfig::default()
                },
            };
            eprintln!("listening on http://127.0.0.1:{port}");
            if config.auth.is_none() {
                eprintln!("auth disabled (--no-auth)");
            }
            tokio::runtime::Runtime::new()?.block_on(osb_server::serve(service, config))?;
        }
    }
    Ok(())
}

fn write_file(root: &Path, relative: &str, contents: &str) -> std::io::Result<()> {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        osb_core::io::create_dir_all(parent)?;
    }
    osb_core::io::write(path, contents)?;
    Ok(())
}
