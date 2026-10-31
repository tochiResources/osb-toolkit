use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use osb_runtime::RuntimeError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors from the service layer.
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),

    #[error("i/o error: {0}")]
    IO(#[from] std::io::Error),

    #[error(
        "converter CLI not found; set OSBTK_CONVERT_CLI or build osb-to-plugin/packages/generate/core"
    )]
    ConverterNotFound,

    #[error("converter CLI failed: {0}")]
    Converter(String),

    #[error("build failed: {0}")]
    Build(String),

    #[error("cancelled")]
    Cancelled,

    #[error("invalid converter output: {0}")]
    Output(String),
}

impl From<osb_core::io::Error> for ServiceError {
    fn from(error: osb_core::io::Error) -> Self {
        ServiceError::IO(error.into())
    }
}

pub type Result<T> = std::result::Result<T, ServiceError>;

/// A request to convert one C# effect script.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertRequest {
    /// The C# effect script text.
    pub script: String,
    /// File name recorded for the script (defaults to `Effect.cs`).
    #[serde(default)]
    pub file_name: Option<String>,
    /// The plug-in target (`after-effects` | `premiere-pro` | `davinci-resolve`).
    pub target: String,
    /// Extra converter CLI flags (e.g. `--bundle`).
    #[serde(default)]
    pub extra_args: Vec<String>,
}

/// A generated file, path relative to the output root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: String,
    pub contents: String,
}

/// A conversion result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertResult {
    pub target: String,
    pub files: Vec<GeneratedFile>,
    pub diagnostics: Vec<serde_json::Value>,
}

/// A request to build an already-generated plug-in crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildRequest {
    /// Directory holding the generated crate's `Cargo.toml`.
    pub crate_dir: String,
    /// The plug-in target (`after-effects` | `premiere-pro` | `davinci-resolve`).
    pub target: String,
    /// Artifact stem (defaults to the crate directory name).
    #[serde(default)]
    pub artifact_name: Option<String>,
    /// Where to write the artifact (defaults to the configured artifact root).
    #[serde(default)]
    pub out_dir: Option<String>,
    /// Extra flags passed to `cargo build`.
    #[serde(default)]
    pub extra_args: Vec<String>,
}

/// How a built artifact is laid out on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactKind {
    File,
    Bundle,
}

/// A built plug-in artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    /// Artifact id; the owning job id once a job produced it.
    pub id: String,
    pub name: String,
    pub target: String,
    pub kind: ArtifactKind,
    pub path: String,
    pub size: u64,
}

/// Where to find the toolchains and how to run them.
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub node: String,
    pub converter_cli: PathBuf,
    pub cargo: String,
    pub artifact_root: PathBuf,
}

impl ServiceConfig {
    /// Auto-detects the bundled Node converter CLI.
    pub fn detect() -> Result<Self> {
        let converter_cli = match std::env::var("OSBTK_CONVERT_CLI") {
            Ok(path) => PathBuf::from(path),
            Err(_) => find_converter_cli().ok_or(ServiceError::ConverterNotFound)?,
        };
        Ok(ServiceConfig {
            node: "node".into(),
            converter_cli,
            cargo: "cargo".into(),
            artifact_root: default_artifact_root(),
        })
    }

    /// A config suitable for in-process operations that do not touch the CLI.
    pub fn in_process() -> Self {
        ServiceConfig {
            node: "node".into(),
            converter_cli: PathBuf::new(),
            cargo: "cargo".into(),
            artifact_root: default_artifact_root(),
        }
    }
}

fn default_artifact_root() -> PathBuf {
    osb_core::io::temp_dir().join("osbtk-artifacts")
}

/// Walks up from the working directory / executable looking for the built CLI.
fn find_converter_cli() -> Option<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(current) = std::env::current_dir() {
        roots.push(current);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.to_path_buf());
        }
    }

    for root in roots {
        let mut dir = Some(root.as_path());
        while let Some(current) = dir {
            let candidate = current
                .join("packages")
                .join("core")
                .join("dist")
                .join("cli.js");
            if candidate.is_file() {
                return Some(candidate);
            }
            dir = current.parent();
        }
    }
    None
}

/// The service: the single implementation behind the CLI and the HTTP API.
#[derive(Debug, Clone)]
pub struct Service {
    pub(crate) config: ServiceConfig,
}

impl Service {
    /// Detects the converter and builds the service.
    pub fn detect() -> Result<Self> {
        Ok(Service {
            config: ServiceConfig::detect()?,
        })
    }

    pub fn with_config(config: ServiceConfig) -> Self {
        Service { config }
    }

    /// Converts an effect script via the bundled Node converter CLI.
    pub fn convert(&self, request: &ConvertRequest) -> Result<ConvertResult> {
        let work = temp_dir("convert")?;
        let file_name = request
            .file_name
            .clone()
            .unwrap_or_else(|| "Effect.cs".into());
        let script_path = work.join(&file_name);
        osb_core::io::write(&script_path, &request.script)?;
        let out_dir = work.join("out");

        let mut command = Command::new(&self.config.node);
        command
            .arg(&self.config.converter_cli)
            .arg(&script_path)
            .arg("--target")
            .arg(&request.target)
            .arg("--out")
            .arg(&out_dir)
            .arg("--json");
        command.args(&request.extra_args);

        let output = command.output()?;
        if !output.status.success() {
            return Err(ServiceError::Converter(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ));
        }

        let parsed: ConverterOutput = serde_json::from_slice(&output.stdout)
            .map_err(|e| ServiceError::Output(e.to_string()))?;

        let mut files = Vec::new();
        for plugin in &parsed.plugins {
            for path in &plugin.files {
                let absolute = if Path::new(path).is_absolute() {
                    PathBuf::from(path)
                } else {
                    out_dir.join(path)
                };
                let contents = osb_core::io::read_string(&absolute).map_err(|error| {
                    ServiceError::Output(format!("{}: {error}", absolute.display()))
                })?;
                files.push(GeneratedFile {
                    path: path.clone(),
                    contents,
                });
            }
        }

        Ok(ConvertResult {
            target: parsed.target,
            files,
            diagnostics: parsed.diagnostics,
        })
    }

    /// Compiles an already-generated crate and packages the host artifact.
    ///
    /// `cancel` is polled while the compiler runs so a queued job can be
    /// aborted; a cancelled build kills the child process.
    pub fn build(&self, request: &BuildRequest, cancel: &AtomicBool) -> Result<Artifact> {
        let crate_dir = PathBuf::from(&request.crate_dir);
        let manifest = crate_dir.join("Cargo.toml");
        if !manifest.is_file() {
            return Err(ServiceError::Build(format!(
                "no Cargo.toml in {}",
                crate_dir.display()
            )));
        }

        let mut command = Command::new(&self.config.cargo);
        command
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(&manifest);
        command.args(&request.extra_args);

        let output = run_cancellable(command, cancel)?;
        if !output.status.success() {
            return Err(ServiceError::Build(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }

        let name = request
            .artifact_name
            .clone()
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| {
                crate_dir
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "plugin".into())
            });
        let release = target_release_dir(&crate_dir);
        let library = find_cdylib(&release, &crate_stem(&crate_dir)).ok_or_else(|| {
            ServiceError::Build(format!("no built library in {}", release.display()))
        })?;
        let out_root = request
            .out_dir
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_else(|| self.config.artifact_root.join(unique_suffix()));

        package(&out_root, &name, &request.target, &library)
    }

    /// Bakes a plain 3D input JSON into baked command-span JSON (in-process).
    pub fn bake3d(&self, input: &str) -> String {
        osb_native::bake_to_json(input)
    }

    /// Bakes a `{ worlds2d, worlds3d, options, options2d }` scene JSON into the
    /// wire `Storyboard` JSON (in-process).
    pub fn bake_scene(&self, input: &str) -> String {
        osb_native::bake_scene_to_json(input)
    }
}

impl crate::jobs::JobRunner for Service {
    fn run(
        &self,
        spec: crate::jobs::JobSpec,
        cancel: &AtomicBool,
    ) -> std::result::Result<Artifact, String> {
        match spec {
            crate::jobs::JobSpec::Build(request) => self
                .build(&request, cancel)
                .map_err(|error| error.to_string()),
        }
    }
}

/// Runs `command`, draining its output on reader threads and polling `cancel`.
fn run_cancellable(mut command: Command, cancel: &AtomicBool) -> Result<std::process::Output> {
    use std::process::Stdio;

    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn()?;
    let stdout_reader = drain(child.stdout.take());
    let stderr_reader = drain(child.stderr.take());

    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ServiceError::Cancelled);
        }
        std::thread::sleep(Duration::from_millis(100));
    };

    let stdout = stdout_reader
        .and_then(|handle| handle.join().ok())
        .unwrap_or_default();
    let stderr = stderr_reader
        .and_then(|handle| handle.join().ok())
        .unwrap_or_default();
    Ok(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

/// Reads a child pipe to the end on a thread so the child never blocks on a
/// full pipe while the parent polls its status.
fn drain<R: std::io::Read + Send + 'static>(
    pipe: Option<R>,
) -> Option<std::thread::JoinHandle<Vec<u8>>> {
    pipe.map(|mut pipe| {
        std::thread::spawn(move || {
            let mut buffer = Vec::new();
            let _ = pipe.read_to_end(&mut buffer);
            buffer
        })
    })
}

/// The `target/release` directory for a crate, preferring one in a parent
/// workspace if the crate is a member.
fn target_release_dir(crate_dir: &Path) -> PathBuf {
    let mut dir = Some(crate_dir);
    while let Some(current) = dir {
        let candidate = current.join("target").join("release");
        if candidate.is_dir() {
            return candidate;
        }
        dir = current.parent();
    }
    crate_dir.join("target").join("release")
}

fn crate_stem(crate_dir: &Path) -> String {
    crate_dir
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Finds a built dynamic library in `dir`, preferring one named after the crate.
fn find_cdylib(dir: &Path, preferred: &str) -> Option<PathBuf> {
    let mut fallback = None;
    for entry in osb_core::io::read_dir(dir).ok()? {
        let path = entry.path;
        if !entry.is_file {
            continue;
        }
        if !matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("dll" | "so" | "dylib")
        ) {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|stem| stem.strip_prefix("lib").unwrap_or(stem))
            .unwrap_or_default();
        if stem == preferred {
            return Some(path);
        }
        fallback.get_or_insert(path);
    }
    fallback
}

/// Copies `library` into the host artifact layout for `target`.
fn package(out_root: &Path, name: &str, target: &str, library: &Path) -> Result<Artifact> {
    let (path, kind) = match target {
        "after-effects" | "premiere-pro" => {
            osb_core::io::create_dir_all(out_root)?;
            let path = out_root.join(format!("{name}.aex"));
            osb_core::io::copy(library, &path)?;
            (path, ArtifactKind::File)
        }
        "davinci-resolve" => {
            let bundle = out_root.join(format!("{name}.ofx.bundle"));
            let contents = bundle.join("Contents").join("Win64");
            osb_core::io::create_dir_all(&contents)?;
            osb_core::io::copy(library, contents.join(format!("{name}.ofx")))?;
            (bundle, ArtifactKind::Bundle)
        }
        other => return Err(ServiceError::Build(format!("unknown target `{other}`"))),
    };

    let size = match kind {
        ArtifactKind::File => osb_core::io::file_size(&path)?,
        ArtifactKind::Bundle => directory_size(&path)?,
    };
    Ok(Artifact {
        id: String::new(),
        name: name.to_string(),
        target: target.to_string(),
        kind,
        path: path.to_string_lossy().into_owned(),
        size,
    })
}

pub(crate) fn directory_size(dir: &Path) -> Result<u64> {
    let mut total = 0;
    for entry in osb_core::io::read_dir(dir)? {
        total += if entry.is_dir {
            directory_size(&entry.path)?
        } else {
            osb_core::io::file_size(&entry.path)?
        };
    }
    Ok(total)
}

#[derive(Deserialize)]
struct ConverterOutput {
    target: String,
    #[serde(default)]
    plugins: Vec<ConverterPlugin>,
    #[serde(default)]
    diagnostics: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct ConverterPlugin {
    #[serde(default)]
    files: Vec<String>,
}

fn temp_dir(prefix: &str) -> Result<PathBuf> {
    let dir = osb_core::io::temp_dir().join(format!("osbtk-{prefix}-{}", unique_suffix()));
    osb_core::io::create_dir_all(&dir)?;
    Ok(dir)
}

fn unique_suffix() -> String {
    format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const BACKGROUND: &str = "using System;\n\
        using osbTKCommon.Scripting;\n\
        using osbTKCommon.Storyboarding;\n\
        namespace osbTKScripts {\n\
        public class Background : StoryboardObjectGenerator {\n\
        public override void Generate() {\n\
        var sprite = GetLayer(\"Background\").CreateSprite(\"sb/bg.png\", OsbOrigin.Centre);\n\
        sprite.Fade(0, 1000, 0, 1);\n\
        }\n\
        }\n\
        }\n";

    fn service() -> Service {
        Service::with_config(ServiceConfig::in_process())
    }

    #[test]
    fn bakes_in_process() {
        let baked = service().bake3d(
            r#"{"nodes":[{"sprite":"sb/dot.png"}],"camera":{"position":[{"time":0,"value":[0,0,5]}]},"options":{"endTime":0}}"#,
        );
        assert!(baked.contains("\"spritePath\":\"sb/dot.png\""));
    }

    #[test]
    fn converts_via_the_node_cli_when_available() {
        let Ok(service) = Service::detect() else {
            return; // no converter CLI in this environment
        };
        let result = service
            .convert(&ConvertRequest {
                script: BACKGROUND.into(),
                file_name: Some("Background.cs".into()),
                target: "after-effects".into(),
                extra_args: vec![],
            })
            .expect("convert");
        assert_eq!(result.target, "after-effects");
        assert!(!result.files.is_empty());
        assert!(result
            .files
            .iter()
            .any(|file| file.path.ends_with("Cargo.toml")));
    }

    #[test]
    fn packages_a_library_for_each_target() {
        let root = temp_dir("package-test").unwrap();
        let library = root.join("libthing.dll");
        osb_core::io::write(&library, b"binary").unwrap();

        let ae = package(&root.join("ae"), "Thing", "after-effects", &library).unwrap();
        assert_eq!(ae.kind, ArtifactKind::File);
        assert!(ae.path.ends_with("Thing.aex"));
        assert_eq!(ae.size, 6);

        let ofx = package(&root.join("ofx"), "Thing", "davinci-resolve", &library).unwrap();
        assert_eq!(ofx.kind, ArtifactKind::Bundle);
        assert!(ofx.path.ends_with("Thing.ofx.bundle"));

        assert!(package(&root, "Thing", "nope", &library).is_err());
        osb_core::io::remove_dir_all(&root).ok();
    }

    #[test]
    fn builds_a_trivial_crate() {
        let root = temp_dir("build-test").unwrap();
        let crate_dir = root.join("probe");
        osb_core::io::create_dir_all(crate_dir.join("src")).unwrap();
        osb_core::io::write(
            crate_dir.join("Cargo.toml"),
            "[package]\nname = \"probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\
             [lib]\ncrate-type = [\"cdylib\"]\n[workspace]\n",
        )
        .unwrap();
        osb_core::io::write(
            crate_dir.join("src/lib.rs"),
            "#[no_mangle]\npub extern \"C\" fn osbtk_probe() {}\n",
        )
        .unwrap();

        let artifact = service()
            .build(
                &BuildRequest {
                    crate_dir: crate_dir.to_string_lossy().into_owned(),
                    target: "after-effects".into(),
                    artifact_name: Some("Probe".into()),
                    out_dir: Some(root.join("out").to_string_lossy().into_owned()),
                    extra_args: vec![],
                },
                &AtomicBool::new(false),
            )
            .expect("build");
        assert!(artifact.path.ends_with("Probe.aex"));
        assert!(Path::new(&artifact.path).is_file());
        osb_core::io::remove_dir_all(&root).ok();
    }
}
