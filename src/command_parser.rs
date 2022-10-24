use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "VCS")]
#[command(version = "0.1.0")]
#[command(about = "Simple tool for code version control", long_about = None)]
pub struct CommandParser {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Prints current repo status")]
    Status,
    #[command(about = "Prints commits list")]
    Log,

    #[command(about = "Initializes repo")]
    Init {
        #[arg(long, value_name("DIRECTORY_PATH"))]
        path: String,
    },

    #[command(about = "Commits current changes")]
    Commit {
        #[arg(long)]
        message: String,
    },

    #[command(arg_required_else_help(true))]
    #[command(about = "Jumps into branch or commit")]
    Jump {
        #[arg(long, value_name("BRANCH_NAME"), conflicts_with("commit"))]
        branch: Option<String>,

        #[arg(long, value_name("COMMIT_HASH"))]
        commit: Option<String>,
    },

    #[command(name("new_branch"))]
    #[command(about = "Creates new branch")]
    NewBranch {
        #[arg(long, value_name("BRANCH_NAME"))]
        name: String,
    },

    #[command(about = "Merges branch into master")]
    Merge {
        #[arg(long, value_name("BRANCH_NAME"))]
        branch: String,
    },
}
