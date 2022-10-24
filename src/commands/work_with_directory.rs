use std::collections::HashSet;
use std::fs;
use std::fs::{copy, create_dir_all, read_dir, read_to_string, remove_dir, remove_file};
use std::path::{Path, PathBuf};
use crate::errors::ErrorType;

#[derive(Default)]
pub struct VcsSnapshotDiff {
    pub(crate) added: Vec<PathBuf>,
    pub(crate) deleted: Vec<PathBuf>,
    pub(crate) modified: Vec<PathBuf>,
}

pub fn find_root_dir(path: &mut PathBuf) -> Result<(), ErrorType> {
    for entry in read_dir(&path)? {
        let path_cur = fs::canonicalize(&entry?.path())?;
        if path_cur.ends_with(".vcs") {
            return Ok(());
        }
    }
    path.pop();
    find_root_dir(path)
}

pub fn get_snapshot_diff(
    old_dir: &Path,
    new_dir: &Path,
) -> Result<VcsSnapshotDiff, ErrorType> {
    let old_snapshot = get_snapshot(&old_dir)?;
    let new_snapshot = get_snapshot(&new_dir)?;
    let mut snapshot_diff = VcsSnapshotDiff::default();
    for old_path in old_snapshot.iter() {
        if new_snapshot.contains(&*old_path) {
            let old_content_path = old_dir.join(old_path);
            let new_content_path = new_dir.join(old_path);
            let old_content = read_to_string(old_content_path)?;
            let new_content = read_to_string(new_content_path)?;
            if old_content != new_content {
                snapshot_diff.modified.push(old_path.clone().to_path_buf());
            }
        } else {
            snapshot_diff.deleted.push(old_path.clone().to_path_buf());
        }
    }
    for new_path in new_snapshot.iter() {
        if !old_snapshot.contains(&*new_path) {
            snapshot_diff.added.push(new_path.clone().to_path_buf());
        }
    }
    Ok(snapshot_diff)
}

pub fn print_snapshot_diff(
    old_dir: &Path,
    new_dir: &Path,
) -> Result<(), ErrorType> {
    let diff = get_snapshot_diff(old_dir, new_dir)?;

    if diff.modified.is_empty() && diff.added.is_empty() && diff.deleted.is_empty() {
        println!("No changes");
        return Ok(());
    }

    for modified in diff.modified {
        let path_gen = new_dir.join(&modified);
        println!("modified: {}", path_gen.to_str().unwrap());
    }
    for added in diff.added {
        let path_gen = new_dir.join(&added);
        println!("added: {}", path_gen.to_str().unwrap());
    }

    for deleted in diff.deleted {
        let path_gen = new_dir.join(&deleted);
        println!("deleted: {}", path_gen.to_str().unwrap());
    }
    Ok(())
}

pub fn get_snapshot(from: &Path) -> Result<HashSet<PathBuf>, ErrorType> {
    let vcs_path = from.join(".vcs");
    let mut file_paths: HashSet<PathBuf> = HashSet::new();
    if from.is_dir() {
        for entry in read_dir(&from)? {
            let entry = entry?;
            let path = entry.path();
            if path != vcs_path {
                if path.is_dir() {
                    let sub_snap = get_snapshot(&path)?;
                    for file_in_sub_snap in sub_snap {
                        file_paths.insert(file_in_sub_snap.strip_prefix(from)?.to_path_buf());
                    }
                } else {
                    file_paths.insert(entry.path().strip_prefix(from)?.to_path_buf());
                }
            }
        }
    }
    Ok(file_paths)
}

pub fn apply_diff(
    diff: &HashSet<PathBuf>,
    dir_from_path: &Path,
    dir_to_path: &Path,
) -> Result<(), ErrorType> {
    for diff_path in diff {
        let from = dir_from_path.join(diff_path);
        let to = dir_to_path.join(diff_path);
        create_dir_all(to.parent().unwrap())?;
        copy(from, to)?;
    }
    Ok(())
}

pub fn erase_dir(path: &Path) -> Result<(), ErrorType> {
    let vcs_path = path.join(".vcs").to_path_buf();
    for entry in read_dir(path)? {
        let path_file = entry?.path();
        if path_file != vcs_path {
            if path_file.is_dir() {
                remove_dir(&path_file)?;
            } else {
                remove_file(&path_file)?;
            }
        }
    }
    Ok(())
}
