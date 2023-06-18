use std::process::Command;

use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

pub fn output_dir() -> Result<Utf8PathBuf> {
    let root = find_root()?;
    let target_dir = find_target_dir(&root)?;

    Ok(target_dir.join(env!("CARGO_PKG_NAME")))
}

fn find_root() -> Result<Utf8PathBuf> {
    #[derive(Deserialize)]
    struct LocateProject {
        root: Utf8PathBuf,
    }

    let output = Command::new("cargo")
        .arg("locate-project")
        .args(["--message-format", "json"])
        .output()?;

    if !output.status.success() {
        bail!(
            "failed running cargo (locate-project):\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    serde_json::from_slice::<LocateProject>(&output.stdout)
        .map(|data| data.root)
        .map_err(Into::into)
}

fn find_target_dir(root: &Utf8Path) -> Result<Utf8PathBuf> {
    #[derive(Deserialize)]
    struct Metadata {
        target_directory: Utf8PathBuf,
    }

    let output = Command::new("cargo")
        .arg("metadata")
        .args(["--format-version", "1"])
        .args(["--manifest-path", root.as_str()])
        .output()?;

    if !output.status.success() {
        bail!(
            "failed running cargo (metadata):\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    serde_json::from_slice::<Metadata>(&output.stdout)
        .map(|data| data.target_directory)
        .map_err(Into::into)
}
