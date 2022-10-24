//Ready to review

use crate::commands::work_with_directory::{
    apply_diff, erase_dir, find_root_dir, get_snapshot, get_snapshot_diff,
};
use crate::info::{
    read_object_from_file, write_object_to_file, BranchesInfo, CommitsInfo, RepoInfo,
};
use std::env::current_dir;
use std::path::PathBuf;
use crate::errors::ErrorType;

pub fn jump_to_commit(commit_hash: &str) -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let commits_info_path = path.join(".vcs").join("commits_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");

    let mut repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;
    let commits_info: CommitsInfo = read_object_from_file(&commits_info_path)?;
    let branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;

    let cur_commit = repo_info.commit.clone();
    let cur_commit_path = path.join(".vcs").join("commits").join(&cur_commit);

    let diff = get_snapshot_diff(&cur_commit_path, &path)?;
    if !diff.modified.is_empty() || !diff.added.is_empty() || !diff.deleted.is_empty() {
        return Err(ErrorType::UnsavedChanges {
            path,
            diff,
        });
    }
    let commit_path = path.join(".vcs").join("commits").join(commit_hash);

    if !commit_path.exists()
        || branches_info.branches[&commits_info.commits[commit_hash].branch].merged
    {
        return Err(ErrorType::NonExistentCommit);
    }

    erase_dir(&path)?;
    apply_diff(&get_snapshot(&commit_path)?, &commit_path, &path)?;

    repo_info.branch = commits_info.commits[commit_hash].branch.clone();
    repo_info.commit = commit_hash.clone().to_string();
    write_object_to_file(&repo_info_path, &repo_info)?;
    println!(
        "Successfully jumped to commit {}. Current branch: {}.",
        commit_hash, &commits_info.commits[commit_hash].branch
    );
    Ok(())
}

pub fn jump_to_branch(branch_name: &String) -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");

    let mut repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;
    let branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;

    if !branches_info.branches.contains_key(branch_name)
        || branches_info.branches[branch_name].merged
    {
        return Err(ErrorType::NonExistentBranch);
    }

    let commit_hash = branches_info.branches[branch_name].last_commit.clone();

    let cur_commit = repo_info.commit.clone();
    let cur_commit_path = path.join(".vcs").join("commits").join(&cur_commit);

    let diff = get_snapshot_diff(&cur_commit_path, &path)?;
    if !diff.modified.is_empty() || !diff.added.is_empty() || !diff.deleted.is_empty() {
        return Err(ErrorType::UnsavedChanges {
            path,
            diff,
        });
    }
    let commit_path = path.join(".vcs").join("commits").join(&commit_hash);

    erase_dir(&path)?;
    apply_diff(&get_snapshot(&commit_path)?, &commit_path, &path)?;

    repo_info.branch = branch_name.clone();
    repo_info.commit = commit_hash.clone();
    write_object_to_file(&repo_info_path, &repo_info)?;
    println!(
        "Successfully jumped to branch {}. Current commit: {}.",
        branch_name, commit_hash
    );
    Ok(())
}
