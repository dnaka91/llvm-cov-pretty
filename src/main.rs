#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    ops::RangeInclusive,
};

use askama::Template;
use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::{Error, Result, WrapErr};
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use time::{OffsetDateTime, UtcOffset};

use self::{
    cli::Cli, highlight::Highlighter, minify::Minifier, schema::JsonExport, templates::FileInfo,
};

mod cargo;
mod cli;
mod highlight;
mod minify;
mod schema;
mod templates;

mod styles {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/styles.rs"));
}

static STYLESHEET: &str = include_str!("../assets/style.css");

fn main() -> Result<()> {
    let cli = Cli::parse();
    let offset = UtcOffset::current_local_offset()?;

    color_eyre::install()?;

    if let Some(sub) = cli.cmd {
        match sub {
            cli::Command::Completions { shell } => cli::completions(shell),
            cli::Command::Manpages { dir } => cli::manpages(&dir)?,
        }

        return Ok(());
    }

    cargo::check_version().wrap_err("failed checking cargo-llvm-cov version")?;

    let JsonExport { data: [export], .. } = if let Some(input) = cli.input {
        let file = BufReader::new(File::open(&input)?);
        serde_json::from_reader::<_, JsonExport>(file)
            .wrap_err_with(|| format!("failed parsing report data from {input:?}"))?
    } else {
        let stdin = std::io::stdin().lock();
        serde_json::from_reader::<_, JsonExport>(stdin)
            .wrap_err("failed parsing report data from STDIN")?
    };

    let project_dir = cargo::project_dir(cli.manifest_path.as_deref())
        .wrap_err("failed to locate project directory")?;
    let output_dir = cargo::output_dir(cli.manifest_path.as_deref())
        .wrap_err("failed to locate output directory")?;

    let files = collect_project_files(&project_dir)?;
    let mut files = merge_file_info(files, &export.files);

    merge_function_info(&mut files, &export.functions);

    files.sort_unstable_by(|a, b| a.relative_path.cmp(&b.relative_path));

    fs::remove_dir_all(&output_dir).ok();
    fs::create_dir_all(&output_dir)?;

    fs::write(output_dir.join("style.css"), STYLESHEET.as_bytes())?;
    fs::write(output_dir.join("syntax.css"), cli.theme.as_bytes())?;

    let minifier = Minifier::new();
    let highlighter = Highlighter::new();

    let index_path = output_dir.join("index.html");

    fs::write(
        &index_path,
        minifier.minify(
            templates::Index {
                title: "Index",
                base_dir: "./",
                generated: OffsetDateTime::now_utc().to_offset(offset),
                files: &files,
                totals: &export.totals,
            }
            .render()?
            .as_bytes(),
        ),
    )?;

    files.into_par_iter().try_for_each(|file| {
        let output = output_dir
            .join(&file.relative_path)
            .with_extension("rs.html");

        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }

        let lines = highlighter.file_to_spans(&file.path, cli.no_highlight)?;

        fs::write(
            output,
            minifier.minify(
                templates::Source {
                    title: file.relative_path.as_str(),
                    base_dir: &"../".repeat(file.relative_path.ancestors().skip(2).count()),
                    lines: &lines,
                    info: &file,
                    coverage_style: cli.coverage_style,
                    show_instantiations: cli.show_instantiations,
                }
                .render()?
                .as_bytes(),
            ),
        )?;

        Ok::<_, Error>(())
    })?;

    if cli.open {
        open::that(index_path)?;
    } else {
        println!(
            "report generated at {:?}",
            index_path.strip_prefix(project_dir).unwrap_or(&index_path)
        );
    }

    Ok(())
}

fn collect_project_files(dir: &Utf8Path) -> Result<Vec<(Utf8PathBuf, Utf8PathBuf)>> {
    let mut files = Vec::new();

    for entry in ignore::Walk::new(dir) {
        let entry = entry?;
        if entry.file_type().map_or(false, |ty| ty.is_file())
            && entry.path().extension().map_or(false, |ext| ext == "rs")
        {
            let absolute = Utf8PathBuf::try_from(entry.into_path())?;
            let relative = absolute.strip_prefix(dir)?.to_owned();
            files.push((absolute, relative));
        }
    }

    Ok(files)
}

fn merge_file_info(
    files: Vec<(Utf8PathBuf, Utf8PathBuf)>,
    coverage: &[schema::File],
) -> Vec<FileInfo> {
    files
        .into_par_iter()
        .filter_map(|(path, relative_path)| {
            let info = coverage.iter().find(|info| {
                if info.filename.is_absolute() {
                    info.filename == path
                } else {
                    info.filename == relative_path
                }
            })?;

            Some(FileInfo {
                path,
                relative_path,
                summary: info.summary.clone(),
                covered: segments_to_ranges(&info.segments)
                    .filter(|&(_, count)| count > 0)
                    .flat_map(|(range, count)| range.map(move |line| (line, count)))
                    .collect(),
                uncovered: segments_to_ranges(&info.segments)
                    .filter_map(|(lines, count)| (count == 0).then_some(lines))
                    .flatten()
                    .collect(),
                called: HashMap::new(),
                uncalled: HashMap::new(),
            })
        })
        .collect()
}

fn segments_to_ranges(
    segments: &[schema::Segment],
) -> impl Iterator<Item = (RangeInclusive<usize>, u64)> + '_ {
    segments.iter().enumerate().filter_map(|(i, seg)| {
        seg.is_start()
            .then(|| segments[i..].iter().find(|seg| seg.is_end()))
            .flatten()
            .map(|end| (seg.line as usize..=end.line as usize, seg.count))
    })
}

fn merge_function_info(files: &mut Vec<FileInfo>, functions: &[schema::Function]) {
    files.par_iter_mut().for_each(|file| {
        for function in functions.iter().filter(|f| f.filenames[0] == file.path) {
            for region in &function.regions {
                if region.execution_count > 0 {
                    for line in region.start.0..=region.end.0 {
                        file.called
                            .entry(line as usize)
                            .or_default()
                            .push((function.name.clone(), region.execution_count));
                    }
                } else {
                    for line in region.start.0..=region.end.0 {
                        file.uncalled
                            .entry(line as usize)
                            .or_default()
                            .push(function.name.clone());
                    }
                }
            }
        }
    });
}
