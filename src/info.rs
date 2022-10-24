use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::errors::ErrorType;

pub fn read_object_from_file<T>(some_path: &PathBuf) -> Result<T, ErrorType>
where
    T: serde::de::DeserializeOwned,
{
    let file = fs::File::open(some_path)?;
    let result = serde_json::from_reader::<_, T>(&file)?;
    Ok(result)
}

pub fn write_object_to_file<T>(
    some_path: &PathBuf,
    object: &T,
) -> Result<(), ErrorType>
where
    T: Serialize,
{
    fs::write(some_path, serde_json::to_string(object)?)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Default)]
pub struct RepoInfo {
    pub(crate) branch: String,
    pub(crate) commit: String,
    pub(crate) amount_of_commits: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommitInfo {
    pub(crate) last_commit: String,
    pub(crate) message: String,
    pub(crate) time: String,
    pub(crate) branch: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommitsInfo {
    pub(crate) commits: HashMap<String, CommitInfo>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BranchInfo {
    pub(crate) last_commit: String,
    pub(crate) last_in_master: String,
    pub(crate) merged: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BranchesInfo {
    pub(crate) branches: HashMap<String, BranchInfo>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommitsList {
    pub(crate) commits: Vec<String>,
}
