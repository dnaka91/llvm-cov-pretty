//! Handling of command line arguments.

use std::{
    fmt::{self, Display},
    fs::OpenOptions,
    io::{self, BufWriter, Write},
};

use camino::{Utf8Path, Utf8PathBuf};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::Shell;
use color_eyre::eyre::{ensure, Result, WrapErr};

use crate::styles::Theme;

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(about, author, version)]
pub struct Cli {
    /// Disable any code highlighting.
    #[arg(long, conflicts_with = "theme")]
    pub no_highlight: bool,
    /// Show annotations for missing instantiations.
    #[arg(long)]
    pub show_instantiations: bool,
    /// The highlighting theme to use, if not disabled.
    #[arg(long, default_value_t = Theme::OneHalf)]
    pub theme: Theme,
    /// Where to place the coverage color marker.
    #[arg(long, default_value_t = CoverageStyle::Line, value_name = "STYLE")]
    pub coverage_style: CoverageStyle,
    /// Location of the project's Cargo.toml, in case the default detection isn't sufficient.
    #[arg(long, value_hint = ValueHint::FilePath, value_name = "PATH")]
    pub manifest_path: Option<Utf8PathBuf>,
    /// Alternative location to save the report files to, overriding the default.
    ///
    /// By default the report is saved to the `<CARGO_TARGET_DIR>/llvm-cov-pretty` directory, where
    /// `<CARGO_TARGET_DIR>` is the output directory as configured for Cargo. This directory can be
    /// influenced by various settings for Cargo as described here:
    ///
    /// https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir
    #[arg(long, value_hint = ValueHint::DirPath, value_name = "PATH")]
    pub output_dir: Option<Utf8PathBuf>,
    /// Open the report in the default application after it's been generated.
    #[arg(long)]
    pub open: bool,
    /// Skip overlaying regular file coverage with function invocation coverage (in source views).
    #[arg(long)]
    pub skip_function_coverage: bool,
    /// Input coverage file encoded as JSON, or STDIN if omitted.
    #[arg(value_hint = ValueHint::FilePath)]
    pub input: Option<Utf8PathBuf>,
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

impl Cli {
    /// Parse the command line arguments passed to the program.
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

/// The way in which to mark source code lines as covered or uncovered.
#[derive(Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum CoverageStyle {
    /// Highlight the whole source line.
    Line,
    /// Only highlight the gutter (count column).
    Gutter,
}

impl Display for CoverageStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Line => "line",
            Self::Gutter => "gutter",
        })
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate auto-completions scripts for various shells.
    Completions {
        /// Shell to generate an auto-completion script for.
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Generate man pages into the given directory.
    Manpages {
        /// Target directory, that must already exist and be empty.
        #[arg(value_hint = ValueHint::DirPath)]
        dir: Utf8PathBuf,
    },
}

/// Generate shell completions for the given shell variant and write the to STDOUT.
pub fn completions(shell: Shell) {
    clap_complete::generate(
        shell,
        &mut Cli::command(),
        env!("CARGO_PKG_NAME"),
        &mut io::stdout().lock(),
    );
}

/// Generate `man` pages and write them into the given directory.
///
/// The output directory must already exist, but if a file with the same name as a man page already
/// exists, an error will be returned. This behavior ensures that we don't accidentally overwrite
/// any existing files (in case the wrong folder was picked by accident).
pub fn manpages(dir: &Utf8Path) -> Result<()> {
    fn print(dir: &Utf8Path, app: &clap::Command) -> Result<()> {
        let name = app.get_display_name().unwrap_or_else(|| app.get_name());
        let out = dir.join(format!("{name}.1"));
        let mut out = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&out)
                .wrap_err_with(|| format!("the file `{out}` already exists"))?,
        );

        clap_mangen::Man::new(app.clone()).render(&mut out)?;
        out.flush()?;

        for sub in app.get_subcommands() {
            print(dir, sub)?;
        }

        Ok(())
    }

    ensure!(dir.try_exists()?, "target directory doesn't exist");

    let mut app = Cli::command();
    app.build();

    print(dir, &app)
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
