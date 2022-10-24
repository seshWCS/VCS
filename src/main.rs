extern crate core;

mod command_parser;
mod commands;
mod errors;
mod info;

use crate::commands::commit::commit;
use crate::commands::init::init;
use crate::commands::jump::{jump_to_branch, jump_to_commit};
use crate::commands::log::log;
use crate::commands::merge::merge;
use crate::commands::new_branch::new_branch;
use crate::commands::status::status;
use crate::Command::{Init, Jump};
use clap::Parser;
use command_parser::Command;
use std::path::PathBuf;
use crate::errors::ErrorType;

fn main() {
    let parser = command_parser::CommandParser::parse();
    match parser.command {
        Init { path } => init(&PathBuf::from(path)).expect("Unable to init"),
        Command::Status => status().expect("Unable to print status"),
        Command::Log => log().expect("Unable to print log"),
        Command::Commit { message } => {
            match commit(&message) {
                Err(ErrorType::InvalidCommit) => {
                    println!("You can create a new commit only from last one.");
                    println!("Aborting...");
                },
                Err(ErrorType::EmptyCommit {branch_name}) => {
                    println!("On branch {}", &branch_name);
                    println!("No changes to be committed");
                },
                Err(_) => println!("Unable to commit"),
                Ok(_) => {},
            }
        },
        Command::NewBranch { name } => {
            match new_branch(&name) {
                Err(ErrorType::InvalidNewBranch) => {
                    println!("Creating a new branch is possible only when you are in the master branch.");
                    println!("Aborting...");
                },
                Err(ErrorType::BranchExists) => {
                    println!("Branch {} already exists.", name);
                    println!("Aborting...");
                },
                Err(_) => println!("Unable to create new branch"),
                Ok(_) => {},
            }
        },
        Command::Merge { branch } => {
            match merge(&branch) {
                Err(ErrorType::InvalidMerge) => {
                    println!("The merge is possible only when you are in the last commit in master.");
                    println!("Aborting...");
                },
                Err(ErrorType::MergeConflict {conflict}) => {
                    println!("Merge conflict: file has been changed both in master and branch");
                    for conflict_file_path in conflict {
                        println!("{}", &conflict_file_path.to_str().unwrap());
                    }
                },
                Err(_) => println!("Unable to merge"),
                Ok(_) => {},
            }
        },
        Jump { branch, commit } => match branch {
            None => {
                match jump_to_commit(&commit.as_ref().unwrap()) {
                    Err(ErrorType::UnsavedChanges {path, diff}) => {
                        println!(
                            "error: Your local changes to the following files should be committed or dropped:"
                        );
                        for modified in diff.modified {
                            let cur_path = path.join(&modified);
                            println!("{}", cur_path.to_str().unwrap());
                        }
                        for added in diff.added {
                            let cur_path = path.join(&added);
                            println!("{}", cur_path.to_str().unwrap());
                        }
                        for deleted in diff.deleted {
                            let cur_path = path.join(&deleted);
                            println!("{}", cur_path.to_str().unwrap());
                        }
                        println!("Please commit your changes or drop them before you jump.");
                        println!("Aborting...");
                    },
                    Err(ErrorType::NonExistentCommit) => {
                        println!("No commit with hash {} exists.", &commit.unwrap());
                        println!("Aborting...");
                    },
                    Err(_) => println!("Unable to jump to commit"),
                    Ok(_) => {},
                }
            },
            Some(branch) => {
                match jump_to_branch(&branch) {
                    Err(ErrorType::NonExistentBranch) => {
                        println!("No branch {} exists.", &branch);
                        println!("Aborting...");
                    },
                    Err(ErrorType::UnsavedChanges {path, diff}) => {
                        println!(
                            "error: Your local changes to the following files should be committed or dropped:"
                        );
                        for modified in diff.modified {
                            let cur_path = path.join(&modified);
                            println!("{}", cur_path.to_str().unwrap());
                        }
                        for added in diff.added {
                            let cur_path = path.join(&added);
                            println!("{}", cur_path.to_str().unwrap());
                        }
                        for deleted in diff.deleted {
                            let cur_path = path.join(&deleted);
                            println!("{}", cur_path.to_str().unwrap());
                        }
                        println!("Please commit your changes or drop them before you jump.");
                        println!("Aborting...");
                    },
                    Err(_) => println!("Unable to jump to branch"),
                    Ok(_) => {},
                }
            },
        },
    }
}
