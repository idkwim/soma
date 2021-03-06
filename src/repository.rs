use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::docker::{self, SomaImage};
use crate::prelude::*;
use crate::problem::{read_manifest, MANIFEST_FILE_NAME};
use crate::repository::backend::{Backend, BackendExt};
use crate::{read_file_contents, NameString};

pub use self::manager::RepositoryManager;

pub mod backend;
mod manager;

const LIST_FILE_NAME: &str = "soma-list.toml";

#[derive(Deserialize)]
struct ProblemList {
    problems: Vec<PathBuf>,
}

impl ProblemList {
    fn sanity_check(&self, repo_path: impl AsRef<Path>) -> SomaResult<()> {
        let hash_set = self
            .problems
            .iter()
            .map(|prob_relative_path| repo_path.as_ref().join(prob_relative_path).canonicalize())
            .collect::<Result<HashSet<_>, _>>()
            .map_err(|_| SomaError::InvalidSomaList)?;

        if hash_set.len() != self.problems.len() {
            Err(SomaError::InvalidSomaList)?;
        }

        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
struct ProblemIndex {
    name: NameString,
    path: PathBuf,
}

pub struct Repository<'a> {
    name: NameString,
    backend: Box<dyn Backend>,
    prob_list: Vec<ProblemIndex>,
    manager: &'a RepositoryManager<'a>,
}

impl<'a> Repository<'a> {
    fn new(
        name: NameString,
        backend: Box<dyn Backend>,
        prob_list: Vec<ProblemIndex>,
        manager: &'a RepositoryManager<'a>,
    ) -> Repository<'a> {
        Repository {
            name,
            backend,
            prob_list,
            manager,
        }
    }

    pub fn name(&self) -> &NameString {
        &self.name
    }

    pub fn backend(&self) -> &dyn Backend {
        &*self.backend
    }

    pub fn manager(&self) -> &'a RepositoryManager {
        &self.manager
    }

    pub fn path(&self) -> PathBuf {
        self.manager.repo_path(&self.name)
    }

    pub fn update(&mut self, images: &[SomaImage]) -> SomaResult<()> {
        let current_prob_set: HashSet<_> = self
            .prob_list
            .clone()
            .into_iter()
            .map(|prob_index| prob_index.name)
            .collect();
        let new_prob_list = {
            let temp_dir = tempfile::tempdir()?;
            self.backend().update_at(temp_dir.path())?;
            read_prob_list(temp_dir.path())?
        };
        let new_prob_set: HashSet<_> = new_prob_list
            .clone()
            .into_iter()
            .map(|prob_index| prob_index.name)
            .collect();

        let existing_problem_removed = current_prob_set
            .difference(&new_prob_set)
            .filter(|prob_name| {
                docker::image_from_repo_and_prob_exists(images, &self.name, prob_name)
            })
            .count()
            > 0;

        if existing_problem_removed {
            Err(SomaError::UnsupportedUpdate)?;
        }

        self.backend.update_at(self.path())?;
        self.prob_list = new_prob_list;
        Ok(())
    }

    pub fn prob_name_iter(&'a self) -> impl Iterator<Item = &'a NameString> {
        self.prob_list.iter().map(|prob_index| &prob_index.name)
    }
}

fn read_prob_manifest(
    repo_path: impl AsRef<Path>,
    prob_relative_path: impl AsRef<Path>,
) -> SomaResult<ProblemIndex> {
    let prob_path = repo_path.as_ref().join(&prob_relative_path);
    let manifest_path = prob_path.join(MANIFEST_FILE_NAME);
    if !manifest_path.exists() {
        Err(SomaError::InvalidRepository)?;
    }

    let manifest = read_manifest(manifest_path)?;
    Ok(ProblemIndex {
        name: manifest.name().clone(),
        path: prob_relative_path.as_ref().to_owned(),
    })
}

fn read_prob_list(repo_path: impl AsRef<Path>) -> SomaResult<Vec<ProblemIndex>> {
    let list_path = repo_path.as_ref().join(LIST_FILE_NAME);
    if list_path.exists() {
        let prob_list: ProblemList = toml::from_slice(&read_file_contents(list_path)?)?;
        prob_list.sanity_check(&repo_path)?;
        prob_list
            .problems
            .iter()
            .map(|prob_relative_path| read_prob_manifest(&repo_path, prob_relative_path))
            .collect()
    } else {
        Ok(vec![read_prob_manifest(&repo_path, "./")?])
    }
}
