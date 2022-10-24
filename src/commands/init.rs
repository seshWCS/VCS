//Ready to review

use crate::commands::work_with_directory::{apply_diff, get_snapshot};
use crate::info::{
    write_object_to_file, BranchInfo, BranchesInfo, CommitInfo, CommitsInfo, CommitsList, RepoInfo,
};
use chrono::Local;
use std::fs::create_dir_all;
use std::path::Path;
use crate::errors::ErrorType;

pub fn init(path: &Path) -> Result<(), ErrorType> {
    let path_of_vcs = path.join(".vcs");
    create_dir_all(&path_of_vcs)?;
    let path_of_commits = path_of_vcs.join("commits"); // directory with commit-files
    create_dir_all(&path_of_commits)?;

    let mut branches_info = BranchesInfo::default();
    let mut commits_info = CommitsInfo::default();
    let mut repo_info = RepoInfo::default();
    let mut commits_list = CommitsList::default();
    let files_list = get_snapshot(&path)?;
    let commit_path = path.join(".vcs").join("commits").join("commit-1");
    create_dir_all(&commit_path)?;
    apply_diff(&files_list, &path, &commit_path)?;
    let cur_time = Local::now();

    repo_info.branch = String::from("master");
    repo_info.commit = String::from("commit-1");
    repo_info.amount_of_commits = 1;

    let commit_info = CommitInfo {
        branch: String::from("master"),
        message: String::from("Initial commit"),
        time: String::from(cur_time.to_string()),
        last_commit: String::from("commit-1"),
    };
    commits_info
        .commits
        .insert(String::from("commit-1"), commit_info);

    let branch_info = BranchInfo {
        last_commit: String::from("commit-1"),
        last_in_master: String::from("commit-1"),
        merged: false,
    };
    branches_info
        .branches
        .insert(String::from("master"), branch_info);

    commits_list.commits.push(String::from("commit-1"));

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let commits_info_path = path.join(".vcs").join("commits_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");
    let commits_list_path = path.join(".vcs").join("commits_list.json");

    write_object_to_file(&repo_info_path, &repo_info)?;
    write_object_to_file(&commits_info_path, &commits_info)?;
    write_object_to_file(&branches_info_path, &branches_info)?;
    write_object_to_file(&commits_list_path, &commits_list)?;

    println!("Initialized VCS repository in {}", &path.to_str().unwrap());
    println!("Created commit:");
    println!("[master commit-1] Initial commit");
    Ok(())
}
