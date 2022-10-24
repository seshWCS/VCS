//Ready to review

use crate::commands::work_with_directory::{find_root_dir, print_snapshot_diff};
use crate::info::{read_object_from_file, BranchesInfo, CommitsInfo, CommitsList};
use std::env::current_dir;
use std::path::PathBuf;
use crate::errors::ErrorType;

pub fn log() -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let commits_info_path = path.join(".vcs").join("commits_info.json");
    let branches_info_path = path.join(".vcs").join("branches_info.json");
    let commits_list_path = path.join(".vcs").join("commits_list.json");

    let commits_info: CommitsInfo = read_object_from_file(&commits_info_path)?;
    let branches_info: BranchesInfo = read_object_from_file(&branches_info_path)?;
    let commits_list: CommitsList = read_object_from_file(&commits_list_path)?;

    for commit in &commits_list.commits {
        let branch = &commits_info.commits[commit].branch;
        if branches_info.branches[branch].merged {
            continue;
        }
        let pred_commit = &commits_info.commits[commit].last_commit;
        let commit_path = path.join(".vcs").join("commits").join(&commit);

        let last_commit_path = path.join(".vcs").join("commits").join(&pred_commit);

        println!("commit {}", &commit);
        println!("Date: {}", &commits_info.commits[commit].time);
        println!("Message: {}", commits_info.commits[commit].message);
        if &commits_info.commits[commit].message != "Initial commit" {
            println!("Changes: ");
        }
        print_snapshot_diff(&last_commit_path, &commit_path)?;
        print!("\n");
    }
    Ok(())
}
