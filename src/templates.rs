#![allow(clippy::trivially_copy_pass_by_ref, clippy::unnecessary_wraps)]

use std::collections::{HashMap, HashSet};

use askama::Template;
use camino::Utf8PathBuf;
use time::OffsetDateTime;

use crate::schema;

const PROJECT: Project = Project {
    name: env!("CARGO_PKG_NAME"),
    repository: env!("CARGO_PKG_REPOSITORY"),
    version: env!("CARGO_PKG_VERSION"),
};

struct Project {
    name: &'static str,
    repository: &'static str,
    version: &'static str,
}

pub struct FileInfo {
    pub path: Utf8PathBuf,
    pub relative_path: Utf8PathBuf,
    pub summary: schema::Summary,
    pub covered: HashMap<usize, u64>,
    pub uncovered: HashSet<usize>,
    pub called: HashMap<usize, Vec<(String, u64)>>,
    pub uncalled: HashMap<usize, Vec<String>>,
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
}

impl<'a> Source<'a> {
    fn get_coverage(&self, index: &usize) -> Coverage {
        self.info
            .covered
            .get(index)
            .copied()
            .map(Coverage::Covered)
            .or_else(|| self.info.uncovered.get(index).map(|_| Coverage::Uncovered))
            .unwrap_or(Coverage::Unknown)
    }
}

#[derive(Clone, Copy)]
pub enum Coverage {
    Covered(u64),
    Uncovered,
    Unknown,
}

#[derive(Clone, Copy)]
pub enum CoverageLevel {
    VeryHigh,
    High,
    Medium,
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
