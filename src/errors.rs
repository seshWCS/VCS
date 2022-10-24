use std::{fmt};
use std::fmt::{Debug, Formatter};
use std::path::{PathBuf, StripPrefixError};
use crate::commands::work_with_directory::VcsSnapshotDiff;

pub enum ErrorType {
    InvalidCommit,
    EmptyCommit {
        branch_name: String,
    },
    UnsavedChanges {
        path: PathBuf,
        diff: VcsSnapshotDiff,
    },
    NonExistentCommit,
    NonExistentBranch,
    InvalidMerge,
    MergeConflict {
      conflict: Vec<PathBuf>,
    },
    InvalidNewBranch,
    BranchExists,
    IoError(std::io::Error),
    SerdeJson(serde_json::Error),
    StripPrefixError(StripPrefixError),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Debug for ErrorType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!();
    }
}

impl std::error::Error for ErrorType {}

impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> Self {
        ErrorType::IoError(err)
    }
}

impl From<StripPrefixError> for ErrorType {
    fn from(err: StripPrefixError) -> Self {
        ErrorType::StripPrefixError(err)
    }
}

impl From<serde_json::Error> for ErrorType {
    fn from(err: serde_json::Error) -> ErrorType {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => ErrorType::IoError(err.into()),
            Category::Syntax | Category::Data | Category::Eof => ErrorType::SerdeJson(err),
        }
    }
}


