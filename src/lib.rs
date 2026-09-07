use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod components;
pub mod svg;
pub mod templates;
pub mod tool;

pub struct BuildSummary {
    pub work_count: usize,
    pub output: PathBuf,
}

/// Build in a temporary directory so invalid input never destroys the last export.
pub fn generate(root: &Path) -> Result<BuildSummary> {
    let dist = root.join("dist");
    fs::create_dir_all(&dist)?;
    let staging = tempfile::Builder::new()
        .prefix("site-build-")
        .tempdir_in(&dist)?;
    let static_output = staging.path().join("static");
    copy_static(&root.join("static"), &static_output, Path::new(""))?;
    let works = website_data::prepare_works(root.join("data/works"), &static_output)?;
    fs::write(
        staging.path().join("index.html"),
        templates::index::render(&works).into_string(),
    )?;
    fs::create_dir_all(staging.path().join("about"))?;
    fs::write(
        staging.path().join("about/index.html"),
        templates::about::render().into_string(),
    )?;

    let output = dist.join("exported");
    if output.exists() {
        fs::remove_dir_all(&output).context("cannot replace previous static export")?;
    }
    fs::rename(staging.path(), &output).context("cannot publish static export")?;
    Ok(BuildSummary {
        work_count: works.len(),
        output,
    })
}

fn copy_static(source: &Path, destination: &Path, relative: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    for entry in
        fs::read_dir(source).with_context(|| format!("cannot read {}", source.display()))?
    {
        let entry = entry?;
        let next = relative.join(entry.file_name());
        // Work assets are generated from TOML, not from old exports or cached copies.
        if next == Path::new("assets/local-works") || next == Path::new("assets/works") {
            continue;
        }
        let target = destination.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_static(&entry.path(), &target, &next)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), &target)
                .with_context(|| format!("cannot copy {}", entry.path().display()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
