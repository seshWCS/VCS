use crate::commands::commit::merge_commit;
use crate::commands::work_with_directory::{apply_diff, find_root_dir, get_snapshot_diff};
use crate::info::{read_object_from_file, BranchesInfo, RepoInfo};
use std::collections::HashSet;
use std::env::current_dir;
use std::path::PathBuf;
use crate::errors::ErrorType;
use crate::errors::ErrorType::{InvalidMerge, MergeConflict};

pub fn merge(branch_name: &str) -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");

    let repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;
    let branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;

    let cur_branch = &repo_info.branch;
    if cur_branch != "master" {
        return Err(InvalidMerge);
    }
    if &repo_info.commit != &branches_info.branches[cur_branch].last_commit {
        return Err(InvalidMerge);
    }

    let branch_commit = &branches_info.branches[branch_name].last_commit;
    let root_commit = &branches_info.branches[branch_name].last_in_master;

    let branch_commit_path = path.join(".vcs").join("commits").join(&branch_commit);

    let root_commit_path = path.join(".vcs").join("commits").join(&root_commit);

    let cur_diff = get_snapshot_diff(&root_commit_path, &path)?;
    let branch_diff = get_snapshot_diff(&root_commit_path, &branch_commit_path)?;

    let mut cur_diff_all = HashSet::new();
    for modified in cur_diff.modified {
        cur_diff_all.insert(modified);
    }
    for added in cur_diff.added {
        cur_diff_all.insert(added);
    }
    for deleted in cur_diff.deleted {
        cur_diff_all.insert(deleted);
    }

    let mut branch_diff_all = HashSet::new();
    let mut conflict = Vec::new();
    for modified in branch_diff.modified {
        if cur_diff_all.remove(&*modified) {
            conflict.push(modified.clone());
        }
        branch_diff_all.insert(modified.clone());
    }
    for added in branch_diff.added {
        if cur_diff_all.remove(&*added) {
            conflict.push(added.clone());
        }
        branch_diff_all.insert(added.clone());
    }
    for deleted in branch_diff.deleted {
        if cur_diff_all.remove(&*deleted) {
            conflict.push(deleted.clone());
        }
        branch_diff_all.insert(deleted.clone());
    }

    if !conflict.is_empty() {
        return Err(MergeConflict {
            conflict,
        })
    }
    apply_diff(&branch_diff_all, &branch_commit_path, &path)?;
    merge_commit(&branch_name)
}
