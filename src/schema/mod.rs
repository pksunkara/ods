use std::path::{Path, PathBuf, absolute};

use clap::Parser;
use eyre::eyre;
use indexmap::IndexMap;
use tracing::{debug, instrument, trace};

use crate::{error::Result, schema::spec::Spec};

pub mod spec;

#[derive(Debug, Parser)]
pub struct SchemaOpt {
    /// Data plan file or folder
    pub plan: PathBuf,
}

impl SchemaOpt {
    #[instrument(name = "load", skip_all)]
    pub fn load(&self) -> Result<IndexMap<String, Spec>> {
        let mut files = IndexMap::new();
        let base_path = absolute(&self.plan)?;

        if self.plan.is_file() {
            trace!("Loading plan from file");
            files.insert(
                self.plan.file_name().unwrap().to_string_lossy().to_string(),
                Spec::load(&self.plan)?,
            );
        } else if self.plan.is_dir() {
            trace!("Loading plan from folder");
            load_dir(&mut files, &self.plan, &base_path)?;
        } else {
            return Err(eyre!("unable to find {}", self.plan.display()));
        }

        debug!("Loaded {} files", files.len());
        Ok(files)
    }
}

fn load_dir(files: &mut IndexMap<String, Spec>, path: &Path, base_path: &PathBuf) -> Result<()> {
    for entry in path.read_dir()? {
        let path = absolute(entry?.path())?;

        if path.is_file() {
            let relative_path = path
                .strip_prefix(base_path)
                .unwrap()
                .to_string_lossy()
                .into();

            trace!("Loading file: {}", relative_path);
            files.insert(relative_path, Spec::load(&path)?);
        } else {
            trace!("Loading folder: {}", path.to_string_lossy());
            load_dir(files, &path, base_path)?;
        }
    }

    Ok(())
}
