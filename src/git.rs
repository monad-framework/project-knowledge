use std::path::Path;
use std::process::Command;

use jiff::Timestamp;

use crate::error::{Error, Result};
use crate::model::{NativeReference, ObservationStatus, SourceObservation};

#[derive(Debug, Default, Clone, Copy)]
pub struct GitAdapter;

impl GitAdapter {
    pub fn observe_repository(&self, root: &Path) -> SourceObservation {
        match git_output(root, ["rev-parse", "--is-inside-work-tree"]) {
            Ok(value) if value.trim() == "true" => {
                let head = git_output(root, ["rev-parse", "HEAD"]).ok();
                SourceObservation {
                    source_system: "git".to_string(),
                    object_type: "repository".to_string(),
                    locator: ".".to_string(),
                    state: head.map(|value| value.trim().to_string()),
                    status: ObservationStatus::Available,
                    observed_at: Timestamp::now().to_string(),
                    detail: None,
                }
            }
            Ok(_) => SourceObservation {
                source_system: "git".to_string(),
                object_type: "repository".to_string(),
                locator: ".".to_string(),
                state: None,
                status: ObservationStatus::NotRepository,
                observed_at: Timestamp::now().to_string(),
                detail: None,
            },
            Err(error) => SourceObservation {
                source_system: "git".to_string(),
                object_type: "repository".to_string(),
                locator: ".".to_string(),
                state: None,
                status: ObservationStatus::NotRepository,
                observed_at: Timestamp::now().to_string(),
                detail: Some(error.to_string()),
            },
        }
    }

    pub fn observe_reference(&self, root: &Path, native: &NativeReference) -> SourceObservation {
        if native.source_system != "git" {
            return SourceObservation {
                source_system: native.source_system.clone(),
                object_type: native.object_type.clone(),
                locator: native.locator.clone(),
                state: None,
                status: ObservationStatus::Unsupported,
                observed_at: Timestamp::now().to_string(),
                detail: Some("M0 only ships a Git source adapter".to_string()),
            };
        }

        let result = match native.object_type.as_str() {
            "repository" => return self.observe_repository(root),
            "blob" => git_output(root, ["rev-parse", &format!("HEAD:{}", native.locator)]),
            "commit" => git_output(root, ["rev-parse", &format!("{}^{{commit}}", native.locator)]),
            other => {
                return SourceObservation {
                    source_system: native.source_system.clone(),
                    object_type: native.object_type.clone(),
                    locator: native.locator.clone(),
                    state: None,
                    status: ObservationStatus::Unsupported,
                    observed_at: Timestamp::now().to_string(),
                    detail: Some(format!("unsupported Git object_type {other}")),
                };
            }
        };

        match result {
            Ok(state) => SourceObservation {
                source_system: native.source_system.clone(),
                object_type: native.object_type.clone(),
                locator: native.locator.clone(),
                state: Some(state.trim().to_string()),
                status: ObservationStatus::Available,
                observed_at: Timestamp::now().to_string(),
                detail: None,
            },
            Err(error) => SourceObservation {
                source_system: native.source_system.clone(),
                object_type: native.object_type.clone(),
                locator: native.locator.clone(),
                state: None,
                status: ObservationStatus::Missing,
                observed_at: Timestamp::now().to_string(),
                detail: Some(error.to_string()),
            },
        }
    }
}

fn git_output<const N: usize>(root: &Path, args: [&str; N]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| Error::Git(format!("unable to execute git: {error}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(Error::Git(if stderr.is_empty() {
            format!("git exited with status {}", output.status)
        } else {
            stderr
        }));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
