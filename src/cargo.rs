use std::process::Command;

use anyhow::{bail, ensure, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

/// Locate the output directory, where the report files are written to.
///
/// This will only work if the current working directory contains a Rust project, as the report is
/// saved under `<target_dir>/cargo-llvm-cov`.
pub fn output_dir() -> Result<Utf8PathBuf> {
    let root = find_root()?;
    let target_dir = find_target_dir(&root)?;

    Ok(target_dir.join(env!("CARGO_PKG_NAME")))
}

/// Use `cargo` to find the root folder of the project under the current working directory.
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

/// Use `cargo` to find the `target` output directory of the given project.
///
/// Similar to how `cargo-llvm-cov` creates custom output folders in the `target` folder, we create
/// our own `target/llvm-cov-pretty` folder that holds the report files. Therefore, we need to find
/// the base `target` folder.
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

/// Ensure the globally installed `cargo-llvm-cov` is a recent _known-to-be-working_ version, to
/// avoid possible errors due to different output in older versions.
pub fn check_version() -> Result<()> {
    use semver::{Comparator, Op, Prerelease, Version};

    static MIN_VERSION: Comparator = Comparator {
        op: Op::GreaterEq,
        major: 0,
        minor: Some(5),
        patch: None,
        pre: Prerelease::EMPTY,
    };

    let output = Command::new("cargo-llvm-cov")
        .args(["llvm-cov", "--version"])
        .output()?;

    if !output.status.success() {
        bail!(
            "failed running cargo-llvm-cov (llvm-cov --version):\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let (name, version) = output
        .trim()
        .split_once(' ')
        .context("no separator between name and version")?;

    ensure!(
        name == "cargo-llvm-cov",
        "program doesn't appear to be cargo-llvm-cov"
    );

    let version = version.parse::<Version>()?;

    ensure!(
        MIN_VERSION.matches(&version),
        "cargo-llvm-cov version {version} is too old, need at least {MIN_VERSION}"
    );

    Ok(())
}
