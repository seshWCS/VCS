//Ready to review

use crate::commands::work_with_directory::{
    apply_diff, find_root_dir, get_snapshot, get_snapshot_diff, print_snapshot_diff,
};
use crate::info::{
    read_object_from_file, write_object_to_file, BranchesInfo, CommitInfo, CommitsInfo,
    CommitsList, RepoInfo,
};
use chrono::Local;
use std::env::current_dir;
use std::path::PathBuf;
use crate::errors::ErrorType;

pub fn commit(message: &str) -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let commits_info_path = path.join(".vcs").join("commits_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");
    let commits_list_path = path.join(".vcs").join("commits_list.json");

    let mut repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;
    let mut commits_info: CommitsInfo = read_object_from_file(&commits_info_path)?;
    let mut branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;
    let mut commits_list: CommitsList = read_object_from_file(&commits_list_path)?;

    let branch_name = repo_info.branch.clone();
    let last_commit = branches_info.branches[&branch_name].last_commit.clone();
    let cur_commit = repo_info.commit.clone();
    if last_commit != cur_commit {
        return Err(ErrorType::InvalidCommit);
    }

    let cur_commit_path = path.join(".vcs").join("commits").join(&cur_commit);

    let diff = get_snapshot_diff(&cur_commit_path, &path)?;
    if diff.modified.is_empty() && diff.added.is_empty() && diff.deleted.is_empty() {
        return Err(ErrorType::EmptyCommit {
            branch_name: branch_name.clone(),
        });
    }

    let mut amount_of_commits = repo_info.amount_of_commits;
    amount_of_commits += 1;

    let mut commit_name = "commit-".to_string();
    commit_name += &*amount_of_commits.to_string();
    let commit_path = path.join(".vcs").join("commits").join(&commit_name);
    apply_diff(&get_snapshot(&path)?, &path, &commit_path)?;

    let time = Local::now().to_string();

    println!("[{}, {}] {}", &branch_name, &commit_name, &message);
    let mut pr = false;
    if !diff.modified.is_empty() {
        if diff.modified.len() == 1 {
            print!("{} file changed", diff.modified.len());
        } else {
            print!("{} files changed", diff.modified.len());
        }
        pr = true;
    }
    if !diff.added.is_empty() {
        if pr {
            print!(", {} added", diff.added.len());
        } else {
            if diff.added.len() == 1 {
                print!("{} file added", diff.added.len());
            } else {
                print!("{} files added", diff.added.len());
            }
            pr = true;
        }
    }
    if !diff.deleted.is_empty() {
        if pr {
            print!(", {} deleted", diff.deleted.len());
        } else {
            if diff.deleted.len() == 1 {
                print!("{} file deleted", diff.deleted.len());
            } else {
                print!("{} files deleted", diff.deleted.len());
            }
        }
    }
    print!("\n");
    print_snapshot_diff(&cur_commit_path, &path)?;

    repo_info.commit = commit_name.clone();
    repo_info.amount_of_commits = amount_of_commits;

    let commit_info = CommitInfo {
        message: message.clone().to_string(),
        branch: branch_name.clone(),
        time: time.clone(),
        last_commit: cur_commit.clone(),
    };
    commits_info
        .commits
        .insert(commit_name.clone(), commit_info);

    branches_info
        .branches
        .get_mut(&branch_name)
        .unwrap()
        .last_commit = commit_name.clone();

    commits_list.commits.push(commit_name.clone());

    write_object_to_file(&repo_info_path, &repo_info)?;
    write_object_to_file(&commits_info_path, &commits_info)?;
    write_object_to_file(&branches_info_path, &branches_info)?;
    write_object_to_file(&commits_list_path, &commits_list)?;
    Ok(())
}

pub fn merge_commit(branch_name: &str) -> Result<(), ErrorType> {
    let mut message: String = String::from("Merged branch ");
    message += branch_name;
    message += ".";
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let commits_info_path = path.join(".vcs").join("commits_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");
    let commits_list_path = path.join(".vcs").join("commits_list.json");

    let mut repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;
    let mut commits_info: CommitsInfo = read_object_from_file(&commits_info_path)?;
    let mut branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;
    let mut commits_list: CommitsList = read_object_from_file(&commits_list_path)?;

    let branch_name_ = repo_info.branch.clone();
    let cur_commit = repo_info.commit.clone();

    let cur_commit_path = path.join(".vcs").join("commits").join(&cur_commit);

    let diff = get_snapshot_diff(&cur_commit_path, &path)?;

    let mut amount_of_commits = repo_info.amount_of_commits;
    amount_of_commits += 1;

    let mut commit_name = "commit-".to_string();
    commit_name += &*amount_of_commits.to_string();
    let commit_path = path.join(".vcs").join("commits").join(&commit_name);
    apply_diff(&get_snapshot(&path)?, &path, &commit_path)?;

    let time = Local::now().to_string();

    repo_info.commit = commit_name.clone();
    repo_info.amount_of_commits = amount_of_commits;

    let commit_info = CommitInfo {
        message: message.clone(),
        branch: branch_name_.clone(),
        time: time.clone(),
        last_commit: cur_commit.clone(),
    };
    commits_info
        .commits
        .insert(commit_name.clone(), commit_info);

    branches_info
        .branches
        .get_mut(&branch_name_)
        .unwrap()
        .last_commit = commit_name.clone();
    branches_info.branches.get_mut(branch_name).unwrap().merged = true;

    commits_list.commits.push(commit_name.clone());

    write_object_to_file(&repo_info_path, &repo_info)?;
    write_object_to_file(&commits_info_path, &commits_info)?;
    write_object_to_file(&branches_info_path, &branches_info)?;
    write_object_to_file(&commits_list_path, &commits_list)?;

    println!("Successfully created merge commit:");
    println!("[{}, {}] {}", &branch_name_, &commit_name, &message);
    let mut pr = false;
    if !diff.modified.is_empty() {
        if diff.modified.len() == 1 {
            print!("{} file changed", diff.modified.len());
        } else {
            print!("{} files changed", diff.modified.len());
        }
        pr = true;
    }
    if !diff.added.is_empty() {
        if pr {
            print!(", {} added", diff.added.len());
        } else {
            if diff.added.len() == 1 {
                print!("{} file added", diff.added.len());
            } else {
                print!("{} files added", diff.added.len());
            }
            pr = true;
        }
    }
    if !diff.deleted.is_empty() {
        if pr {
            print!(", {} deleted", diff.deleted.len());
        } else {
            if diff.deleted.len() == 1 {
                print!("{} file deleted", diff.deleted.len());
            } else {
                print!("{} files deleted", diff.deleted.len());
            }
        }
        pr = true;
    }
    if !pr {
        println!("No changes to be committed");
    }
    print!("\n");
    print_snapshot_diff(&cur_commit_path, &path)?;
    println!("Deleted {}", &branch_name);
    Ok(())
}
