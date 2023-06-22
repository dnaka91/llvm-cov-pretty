use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::{
    eyre::{ensure, eyre, Context, Result},
    Help, SectionExt,
};
use serde::Deserialize;

/// Locate the root directory of the project under the current working directory.
pub fn project_dir(manifest_path: Option<&Utf8Path>) -> Result<Utf8PathBuf> {
    let manifest_path = match manifest_path {
        Some(path) => path.to_owned(),
        None => cargo_locate_project().wrap_err("failed to locate project")?,
    };

    cargo_metadata(&manifest_path)
        .map(|meta| meta.workspace_root)
        .wrap_err("failed to load project metadata")
}

/// Locate the output directory, where the report files are written to.
///
/// Similar to how `cargo-llvm-cov` creates custom output folders in the `target` folder, we create
/// our own `target/llvm-cov-pretty` folder that holds the report files.
///
/// This will only work if the current working directory contains a Rust project.
pub fn output_dir(manifest_path: Option<&Utf8Path>) -> Result<Utf8PathBuf> {
    let manifest_path = match manifest_path {
        Some(path) => path.to_owned(),
        None => cargo_locate_project().wrap_err("failed to locate project")?,
    };

    cargo_metadata(&manifest_path)
        .map(|meta| meta.target_directory.join(env!("CARGO_PKG_NAME")))
        .wrap_err("failed to load project metadata")
}

/// Use `cargo` to find the root `Cargo.toml` file of the project under the current working
/// directory.
fn cargo_locate_project() -> Result<Utf8PathBuf> {
    #[derive(Deserialize)]
    struct LocateProject {
        root: Utf8PathBuf,
    }

    let output = Command::new("cargo")
        .arg("locate-project")
        .args(["--message-format", "json", "--workspace"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("failed running cargo (locate-project)")
            .with_section(move || stderr.to_string().header("Stderr:"))
            .suggestion("ensure you are inside a Rust project"));
    }

    serde_json::from_slice::<LocateProject>(&output.stdout)
        .map(|data| data.root)
        .wrap_err("failed to parse the `cargo locate-project` JSON output")
}

/// Partial structure for the `cargo metadata` JSON output.
#[derive(Deserialize)]
struct Metadata {
    /// The target directory where build artifacts are stored.
    target_directory: Utf8PathBuf,
    /// The root of the workspace (even if the project isn't a workspace).
    workspace_root: Utf8PathBuf,
}

/// Use `cargo` to get the metadata information of the given project.
fn cargo_metadata(manifest_path: &Utf8Path) -> Result<Metadata> {
    let output = Command::new("cargo")
        .arg("metadata")
        .args(["--format-version", "1"])
        .args(["--manifest-path", manifest_path.as_str()])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("failed running cargo (metadata)")
            .with_section(move || stderr.to_string().header("Stderr:")));
    }

    serde_json::from_slice::<Metadata>(&output.stdout)
        .wrap_err("failed to parse the `cargo metadata` JSON output")
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
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("failed running cargo-llvm-cov (llvm-cov --version)")
            .with_section(move || stdout.to_string().header("Stdout:"))
            .with_section(move || stderr.to_string().header("Stderr:"))
            .suggestion("ensure it is installed and available on your $PATH"));
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let (name, version) = output
        .trim()
        .split_once(' ')
        .ok_or_else(|| eyre!("no separator between name and version"))?;

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

#[cfg(test)]
mod tests {
    #[test]
    fn project_dir() {
        super::project_dir(None).unwrap();
    }

    #[test]
    fn output_dir() {
        super::output_dir(None).unwrap();
    }

    #[test]
    fn cargo_locate_project() {
        super::cargo_locate_project().unwrap();
    }

    #[test]
    fn cargo_metadata() {
        let root = super::cargo_locate_project().unwrap();
        super::cargo_metadata(&root).unwrap();
    }

    #[test]
    fn check_version() {
        super::check_version().unwrap();
    }
}
