use camino::Utf8PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonExport {
    pub version: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub data: [Export; 1],
}

#[derive(Deserialize)]
pub struct Export {
    pub files: Vec<File>,
    #[serde(default)]
    pub functions: Vec<Function>,
    pub totals: Summary,
}

#[derive(Deserialize)]
pub struct File {
    pub filename: Utf8PathBuf,
    #[serde(default)]
    pub segments: Vec<Segment>,
    #[serde(default)]
    pub branches: Vec<BranchRegion>,
    #[serde(default)]
    pub expansions: Vec<Expansion>,
    pub summary: Summary,
}

pub struct Segment {
    pub line: u64,
    pub col: u64,
    pub count: u64,
    pub has_count: bool,
    pub is_region_entry: bool,
    pub is_gap_region: bool,
}

impl<'de> Deserialize<'de> for Segment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Array(u64, u64, u64, bool, bool, bool);

        Array::deserialize(deserializer).map(|a| Self {
            line: a.0,
            col: a.1,
            count: a.2,
            has_count: a.3,
            is_region_entry: a.4,
            is_gap_region: a.5,
        })
    }
}

impl Segment {
    pub fn is_start(&self) -> bool {
        self.has_count && self.is_region_entry && !self.is_gap_region
    }

    pub fn is_end(&self) -> bool {
        !self.has_count && !self.is_region_entry && !self.is_gap_region
    }
}

#[derive(Deserialize)]
pub struct Function {
    #[serde(with = "demangle")]
    pub name: String,
    pub count: u64,
    pub regions: Vec<Region>,
    pub branches: Vec<BranchRegion>,
    pub filenames: Vec<Utf8PathBuf>,
}

pub struct Region {
    pub line_start: u64,
    pub column_start: u64,
    pub line_end: u64,
    pub column_end: u64,
    pub execution_count: u64,
    pub file_id: u64,
    pub expanded_file_id: u64,
    pub kind: RegionKind,
}

pub enum RegionKind {
    Code,
    Expansion,
    Skipped,
    Gap,
    Branch,
    Unknown(u8),
}

impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Array(u64, u64, u64, u64, u64, u64, u64, u8);

        Array::deserialize(deserializer).map(|a| Self {
            line_start: a.0,
            column_start: a.1,
            line_end: a.2,
            column_end: a.3,
            execution_count: a.4,
            file_id: a.5,
            expanded_file_id: a.6,
            kind: match a.7 {
                0 => RegionKind::Code,
                1 => RegionKind::Expansion,
                2 => RegionKind::Skipped,
                3 => RegionKind::Gap,
                4 => RegionKind::Branch,
                v => RegionKind::Unknown(v),
            },
        })
    }
}

pub struct BranchRegion {
    pub line_start: u64,
    pub column_start: u64,
    pub line_end: u64,
    pub column_end: u64,
    pub execution_count: u64,
    pub false_execution_count: u64,
    pub file_id: u64,
    pub expanded_file_id: u64,
    pub kind: RegionKind,
}

impl<'de> Deserialize<'de> for BranchRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Array(u64, u64, u64, u64, u64, u64, u64, u64, u8);

        Array::deserialize(deserializer).map(|a| Self {
            line_start: a.0,
            column_start: a.1,
            line_end: a.2,
            column_end: a.3,
            execution_count: a.4,
            false_execution_count: a.5,
            file_id: a.6,
            expanded_file_id: a.7,
            kind: match a.8 {
                0 => RegionKind::Code,
                1 => RegionKind::Expansion,
                2 => RegionKind::Skipped,
                3 => RegionKind::Gap,
                4 => RegionKind::Branch,
                v => RegionKind::Unknown(v),
            },
        })
    }
}

#[derive(Deserialize)]
pub struct Expansion {
    pub filenames: Vec<Utf8PathBuf>,
    pub source_region: Region,
    pub target_regions: Vec<Region>,
    pub branches: Vec<BranchRegion>,
}

#[derive(Clone, Deserialize)]
pub struct Summary {
    pub lines: CoverageCounts,
    pub functions: CoverageCounts,
    pub instantiations: CoverageCounts,
    pub regions: CoverageCounts2,
    pub branches: CoverageCounts2,
}

#[derive(Clone, Deserialize)]
pub struct CoverageCounts {
    pub count: u64,
    pub covered: u64,
    pub percent: f64,
}

#[derive(Clone, Deserialize)]
pub struct CoverageCounts2 {
    pub count: u64,
    pub covered: u64,
    pub notcovered: u64,
    pub percent: f64,
}

mod demangle {
    use std::fmt;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ValueVisitor)
    }

    struct ValueVisitor;

    impl<'de> serde::de::Visitor<'de> for ValueVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("possibly mangled rust identifier")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(format!("{:#}", rustc_demangle::demangle(v)))
        }
    }
}
