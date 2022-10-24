//Ready to review

use crate::commands::work_with_directory::find_root_dir;
use crate::info::{
    read_object_from_file, write_object_to_file, BranchInfo, BranchesInfo, RepoInfo,
};
use std::env::current_dir;
use std::path::PathBuf;
use crate::errors::ErrorType;
use crate::errors::ErrorType::{BranchExists, InvalidNewBranch};

pub fn new_branch(name: &str) -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");

    let mut repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;
    let mut branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;

    let cur_branch = repo_info.branch.clone();
    if cur_branch != "master" {
        return Err(InvalidNewBranch);
    }
    if branches_info.branches.contains_key(name) {
        return Err(BranchExists);
    }

    let cur_commit = repo_info.commit.clone();

    let branch_info = BranchInfo {
        last_commit: cur_commit.clone(),
        last_in_master: cur_commit.clone(),
        merged: false,
    };
    branches_info
        .branches
        .insert(name.clone().to_string(), branch_info);

    repo_info.branch = name.clone().to_string();

    write_object_to_file(&repo_info_path, &repo_info)?;
    write_object_to_file(&branches_info_path, &branches_info)?;

    println!(
        "Created a new branch {} from master's commit {}",
        &name, &cur_commit
    );
    Ok(())
}
