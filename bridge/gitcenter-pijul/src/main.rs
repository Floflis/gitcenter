//! GitCenter's narrow, authenticated bridge to a pinned Pijul CLI.
//!
//! The browser never invokes this process directly. A trusted local service
//! starts it with a repository root and a high-entropy shared secret.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Output};

const PROTOCOL_VERSION: u32 = 1;
const PIJUL_VERSION: &str = "1.0.0-beta.24";
const TOKEN_ENV: &str = "GITCENTER_PIJUL_TOKEN";
const ROOT_ENV: &str = "GITCENTER_PIJUL_ROOT";
const BIN_ENV: &str = "GITCENTER_PIJUL_BIN";

#[derive(Debug, Deserialize)]
struct Request {
    protocol_version: u32,
    request_id: String,
    token: String,
    operation: String,
    #[serde(default)]
    repository: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    protocol_version: u32,
    request_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<BridgeError>,
}

#[derive(Debug, Serialize)]
struct BridgeError {
    code: &'static str,
    message: String,
}

struct Config {
    root: PathBuf,
    token: String,
    pijul_bin: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("gitcenter-pijul: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let config = load_config()?;
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    for line in stdin.lock().lines() {
        let line = line.map_err(|error| format!("cannot read request: {error}"))?;
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle_request(&config, request),
            Err(error) => Response {
                protocol_version: PROTOCOL_VERSION,
                request_id: String::new(),
                ok: false,
                result: None,
                error: Some(BridgeError {
                    code: "invalid_request",
                    message: format!("invalid JSON request: {error}"),
                }),
            },
        };

        serde_json::to_writer(&mut stdout, &response)
            .map_err(|error| format!("cannot encode response: {error}"))?;
        stdout.write_all(b"\n").map_err(|error| error.to_string())?;
        stdout.flush().map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn load_config() -> Result<Config, String> {
    let root = env::var_os(ROOT_ENV)
        .ok_or_else(|| format!("{ROOT_ENV} must point to the managed repository root"))?;
    let root = fs::canonicalize(root)
        .map_err(|error| format!("cannot resolve repository root: {error}"))?;
    if !root.is_dir() {
        return Err("configured repository root is not a directory".into());
    }

    let token = env::var(TOKEN_ENV)
        .map_err(|_| format!("{TOKEN_ENV} must contain a high-entropy secret"))?;
    if token.len() < 32 {
        return Err(format!("{TOKEN_ENV} must contain at least 32 bytes"));
    }

    Ok(Config {
        root,
        token,
        pijul_bin: env::var(BIN_ENV).unwrap_or_else(|_| "pijul".into()),
    })
}

fn handle_request(config: &Config, request: Request) -> Response {
    let request_id = request.request_id.clone();

    let result = (|| {
        if request.protocol_version != PROTOCOL_VERSION {
            return Err(("unsupported_protocol", format!(
                "protocol version {} is unsupported; expected {}",
                request.protocol_version, PROTOCOL_VERSION
            )));
        }
        if !constant_time_eq(request.token.as_bytes(), config.token.as_bytes()) {
            return Err(("unauthorized", "authentication failed".into()));
        }

        match request.operation.as_str() {
            "version" => Ok(json!({
                "bridge_version": env!("CARGO_PKG_VERSION"),
                "protocol_version": PROTOCOL_VERSION,
                "pijul_version": PIJUL_VERSION
            })),
            "inspect" => {
                let repository = resolve_existing_repository(config, request.repository.as_deref())?;
                inspect_repository(config, &repository)
            }
            "init" => {
                // Refuse an incompatible CLI before any filesystem mutation.
                check_pijul_version(config)?;
                let repository = resolve_init_target(config, request.repository.as_deref())?;
                if fs::read_dir(&repository)
                    .map_err(|error| ("filesystem_error", error.to_string()))?
                    .next()
                    .is_some()
                {
                    return Err(("repository_not_empty", "init target must be an empty directory".into()));
                }

                run_pijul(config, [
                    "init".to_string(),
                    "--channel".to_string(),
                    "main".to_string(),
                    repository.to_string_lossy().into_owned(),
                ])?;
                inspect_repository(config, &repository)
            }
            _ => Err(("unsupported_operation", format!(
                "unsupported operation: {}", request.operation
            ))),
        }
    })();

    match result {
        Ok(value) => Response {
            protocol_version: PROTOCOL_VERSION,
            request_id,
            ok: true,
            result: Some(value),
            error: None,
        },
        Err((code, message)) => Response {
            protocol_version: PROTOCOL_VERSION,
            request_id,
            ok: false,
            result: None,
            error: Some(BridgeError { code, message }),
        },
    }
}

fn resolve_existing_repository(config: &Config, relative: Option<&str>) -> Result<PathBuf, (&'static str, String)> {
    let relative = validate_relative_path(relative)?;
    let candidate = config.root.join(relative);
    let canonical = fs::canonicalize(&candidate)
        .map_err(|error| ("repository_not_found", format!("cannot resolve repository: {error}")))?;
    ensure_within_root(&config.root, &canonical)?;

    let marker = canonical.join(".pijul");
    let marker_real = fs::canonicalize(&marker)
        .map_err(|_| ("not_a_pijul_repository", "repository has no readable .pijul directory".into()))?;
    if !marker_real.is_dir() {
        return Err(("not_a_pijul_repository", ".pijul is not a directory".into()));
    }
    ensure_within_root(&canonical, &marker_real)?;
    Ok(canonical)
}

fn resolve_init_target(config: &Config, relative: Option<&str>) -> Result<PathBuf, (&'static str, String)> {
    let relative = validate_relative_path(relative)?;
    if !relative.components().any(|component| matches!(component, Component::Normal(_))) {
        return Err(("invalid_path", "initialization target must be below the managed root".into()));
    }

    let candidate = config.root.join(relative);
    let canonical = fs::canonicalize(&candidate)
        .map_err(|error| ("repository_not_found", format!("init target must already exist: {error}")))?;
    ensure_within_root(&config.root, &canonical)?;
    if !canonical.is_dir() {
        return Err(("invalid_path", "init target must be a directory".into()));
    }
    Ok(canonical)
}

fn validate_relative_path(path: Option<&str>) -> Result<PathBuf, (&'static str, String)> {
    let path = path.ok_or_else(|| ("missing_repository", "repository is required".into()))?;
    if path.is_empty() {
        return Err(("invalid_path", "repository path cannot be empty".into()));
    }

    let candidate = Path::new(path);
    if candidate.components().any(|component| {
        !matches!(component, Component::Normal(_) | Component::CurDir)
    }) {
        return Err(("invalid_path", "repository must be a relative path without parent traversal".into()));
    }
    Ok(candidate.to_path_buf())
}

fn ensure_within_root(root: &Path, path: &Path) -> Result<(), (&'static str, String)> {
    if !path.starts_with(root) {
        return Err(("path_outside_root", "repository path escapes the configured root".into()));
    }
    Ok(())
}

fn check_pijul_version(config: &Config) -> Result<(), (&'static str, String)> {
    let version = run_pijul(config, ["--version".to_string()])?;
    let version_text = String::from_utf8_lossy(&version.stdout).trim().to_string();
    if !version_text.split_whitespace().any(|part| part == PIJUL_VERSION) {
        return Err(("unsupported_pijul_version", format!(
            "expected Pijul {PIJUL_VERSION}, found: {version_text}"
        )));
    }
    Ok(())
}

fn inspect_repository(config: &Config, repository: &Path) -> Result<Value, (&'static str, String)> {
    check_pijul_version(config)?;

    // The documented channel-list command is parsed only inside this adapter.
    let channels_output = run_pijul(config, [
        "channel".to_string(),
        "--repository".to_string(),
        repository.to_string_lossy().into_owned(),
    ])?;
    let channels_text = String::from_utf8_lossy(&channels_output.stdout);
    let mut channels = Vec::new();
    let mut current_channel: Option<String> = None;
    for line in channels_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (current, name) = match line.strip_prefix("* ") {
            Some(name) => (true, name.trim()),
            None => (false, line),
        };
        if name.is_empty() || name.chars().any(char::is_control) {
            return Err(("invalid_pijul_output", "Pijul returned an invalid channel name".into()));
        }
        if current {
            current_channel = Some(name.to_string());
        }
        channels.push(name.to_string());
    }

    let log_output = run_pijul(config, [
        "log".to_string(),
        "--repository".to_string(),
        repository.to_string_lossy().into_owned(),
        "--hash-only".to_string(),
        "--limit".to_string(),
        "1".to_string(),
    ])?;
    let latest_change = String::from_utf8_lossy(&log_output.stdout)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string);

    Ok(json!({
        "repository": repository.strip_prefix(&config.root)
            .unwrap_or(repository).to_string_lossy(),
        "pijul_version": PIJUL_VERSION,
        "channels": channels,
        "current_channel": current_channel,
        "latest_change": latest_change
    }))
}

fn run_pijul<I, S>(config: &Config, args: I) -> Result<Output, (&'static str, String)>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let output = Command::new(&config.pijul_bin)
        .args(args)
        .output()
        .map_err(|error| ("pijul_unavailable", format!("cannot execute configured Pijul binary: {error}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message: String = stderr.chars().take(2048).collect();
        return Err(("pijul_command_failed", format!(
            "Pijul exited with {}: {}", output.status, message.trim()
        )));
    }
    Ok(output)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let max_len = left.len().max(right.len());
    let mut difference = left.len() ^ right.len();
    for index in 0..max_len {
        let a = left.get(index).copied().unwrap_or(0);
        let b = right.get(index).copied().unwrap_or(0);
        difference |= usize::from(a ^ b);
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_relative_paths_without_traversal() {
        assert!(validate_relative_path(Some("projects/demo")).is_ok());
        assert!(validate_relative_path(Some(".")).is_ok());
        assert!(validate_relative_path(Some("../outside")).is_err());
        assert!(validate_relative_path(Some("/tmp/repo")).is_err());
        assert!(validate_relative_path(Some("projects/../../outside")).is_err());
    }

    #[test]
    fn compares_tokens_without_early_exit() {
        assert!(constant_time_eq(b"same-secret", b"same-secret"));
        assert!(!constant_time_eq(b"same-secret", b"other-secret"));
        assert!(!constant_time_eq(b"short", b"longer"));
    }

    #[test]
    fn rejects_paths_outside_root() {
        let root = Path::new("/srv/gitcenter");
        assert!(ensure_within_root(root, Path::new("/srv/gitcenter/repo")).is_ok());
        assert!(ensure_within_root(root, Path::new("/srv/other/repo")).is_err());
    }
}
