#![allow(clippy::trivially_copy_pass_by_ref, clippy::unnecessary_wraps)]

use std::collections::{HashMap, HashSet, BTreeSet};

use askama::Template;
use camino::Utf8PathBuf;
use time::OffsetDateTime;

use crate::{cli::CoverageStyle, schema};

/// Global constant instance with this project's info, so it doesn't have to be included as part of
/// each template struct.
const PROJECT: Project = Project {
    name: env!("CARGO_PKG_NAME"),
    repository: env!("CARGO_PKG_REPOSITORY"),
    version: env!("CARGO_PKG_VERSION"),
};

/// Basic information about this project, and not the project being invoked on. This data is used
/// throughout the templates to generate footer information at the end of the content.
struct Project {
    /// Name of this project (not the project the report is generated for).
    name: &'static str,
    /// Source code location.
    repository: &'static str,
    /// Current version.
    version: &'static str,
}

/// Location and coverage information of a single file.
pub struct FileInfo {
    /// Absolute path to the file.
    pub path: Utf8PathBuf,
    /// File path relative to the project root.
    pub relative_path: Utf8PathBuf,
    /// Coverage information that sums up the information of all files.
    pub summary: schema::Summary,
    /// Mapping from source lines to coverage hit counts.
    pub covered: HashMap<usize, u64>,
    /// Set of uncovered lines.
    pub uncovered: HashSet<usize>,
    /// Mapping from source lines to instantiated function calls and their hit counts.
    pub called: HashMap<usize, Vec<(String, u64)>>,
    /// Set of uninstantiated function calls.
    pub uncalled: HashMap<usize, BTreeSet<String>>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub title: &'a str,
    pub base_dir: &'a str,
    pub generated: OffsetDateTime,
    pub files: &'a [FileInfo],
    pub totals: &'a schema::Summary,
}

#[derive(Template)]
#[template(path = "source.html")]
pub struct Source<'a> {
    pub title: &'a str,
    pub base_dir: &'a str,
    pub lines: &'a [String],
    pub info: &'a FileInfo,
    pub coverage_style: CoverageStyle,
    pub show_instantiations: bool,
    pub overlay_instantations: bool,
}

impl<'a> Source<'a> {
    fn get_coverage(&self, index: &usize) -> Coverage {
        self.info
            .covered
            .get(index)
            .copied()
            .map(|count| {
                // Even though the line is covered, we might have a part of it that is not covered.
                // We keep the original coverage count but still mark it as uncovered.
                if self.overlay_instantations && self.info.uncalled.contains_key(index) {
                    Coverage::Uncovered(count)
                } else {
                    Coverage::Covered(count)
                }
            })
            .or_else(|| {
                self.info
                    .uncovered
                    .get(index)
                    .map(|_| Coverage::Uncovered(0))
            })
            .unwrap_or(Coverage::Unknown)
    }
}

/// The coverage inforamation for a single line of code.
#[derive(Clone, Copy)]
pub enum Coverage {
    /// Line is covered and was called the amount of times.
    Covered(u64),
    /// Line is uncovered but can still have a count, if only parts of the line are uncovered.
    Uncovered(u64),
    /// The coverage information is not available.
    Unknown,
}

/// Quality level of coverage.
///
/// This is somewhat of a categorization of different coverage percentages, and is mostly there for
/// coloring purposes.
#[derive(Clone, Copy)]
pub enum CoverageLevel {
    /// 90% coverage and upwards.
    VeryHigh,
    /// 75% or more.
    High,
    /// 50% at least.
    Medium,
    /// Anything else.
    Low,
}

mod filters {
    use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};

    use super::CoverageLevel;

    pub fn format_datetime(value: &OffsetDateTime) -> Result<String, askama::Error> {
        static FORMAT: &[FormatItem<'_>] =
            format_description!("[year]-[month]-[day] [hour]:[minute]");
        value
            .format(FORMAT)
            .map_err(|e| askama::Error::Custom(e.into()))
    }

    pub fn coverage_level(value: &f64) -> Result<CoverageLevel, askama::Error> {
        Ok(match *value {
            v if v > 90.0 => CoverageLevel::VeryHigh,
            v if v > 75.0 => CoverageLevel::High,
            v if v > 50.0 => CoverageLevel::Medium,
            _ => CoverageLevel::Low,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use askama::Template;
    use camino::Utf8PathBuf;
    use time::OffsetDateTime;

    use super::{schema, CoverageStyle, FileInfo};

    #[test]
    fn render_index() {
        super::Index {
            title: "",
            base_dir: "",
            generated: OffsetDateTime::UNIX_EPOCH,
            files: &[FileInfo {
                path: Utf8PathBuf::from("/home/user/project/src/file.rs"),
                relative_path: Utf8PathBuf::from("src/file.rs"),
                summary: schema::Summary::default(),
                covered: HashMap::default(),
                uncovered: HashSet::default(),
                called: HashMap::default(),
                uncalled: HashMap::default(),
            }],
            totals: &schema::Summary::default(),
        }
        .render()
        .unwrap();
    }

    #[test]
    fn render_source() {
        super::Source {
            title: "",
            base_dir: "",
            lines: &[String::from("test")],
            info: &FileInfo {
                path: Utf8PathBuf::from("/home/user/project/src/file.rs"),
                relative_path: Utf8PathBuf::from("src/file.rs"),
                summary: schema::Summary::default(),
                covered: HashMap::default(),
                uncovered: HashSet::default(),
                called: HashMap::default(),
                uncalled: HashMap::default(),
            },
            coverage_style: CoverageStyle::Line,
            show_instantiations: true,
            overlay_instantations: true,
        }
        .render()
        .unwrap();
    }
}
