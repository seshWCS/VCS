//Ready to review

use crate::commands::work_with_directory::{find_root_dir, get_snapshot_diff, print_snapshot_diff};
use crate::info::{read_object_from_file, RepoInfo};
use std::env::current_dir;
use std::path::PathBuf;
use crate::errors::ErrorType;

pub fn status() -> Result<(), ErrorType> {
    let mut path: PathBuf = current_dir()?;
    find_root_dir(&mut path)?;

    let repo_info_path = path.join(".vcs").join("repo_info.json");

    let repo_info: RepoInfo = read_object_from_file(&repo_info_path)?;

    let cur_commit = &repo_info.commit;
    let branch_name = &repo_info.branch;

    let commit_path = path.join(".vcs").join("commits").join(&cur_commit);

    let diff = get_snapshot_diff(&commit_path, &path)?;
    if diff.modified.is_empty() && diff.added.is_empty() && diff.deleted.is_empty() {
        println!("On branch {}", branch_name);
        println!("No changes to be committed");
    } else {
        println!("On branch {}", branch_name);
        println!("Changes to be committed:");
        print_snapshot_diff(&commit_path, &path)?;
    }
    Ok(())
}
