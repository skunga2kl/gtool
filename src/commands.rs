use crate::git::{run_git, run_status};
use colored::Colorize;

pub fn run_quick(
    message: String,
    nopush: bool,
    amend: bool,
    all: bool,
    files: Vec<String>,
    sync: bool
) {
    if sync {
        println!("Syncing (pull rebase)...");
        run_git(&["pull", "--rebase"]);
    }

    println!("Adding...");

    if !files.is_empty() {
        let file_refs: Vec<&str> = files.iter().map(|f| f.as_str()).collect();
        run_git(&["add"]);
        run_git(&file_refs);
    } else if all {
        run_git(&["add", "-A"]);
    } else {
        run_git(&["add", "."]);
    }

    println!("{}", "Committing...".green());
    if amend {
        run_git(&["commit", "--amend", "-m", &message]);
    } else if all {
        run_git(&["commit", "-a", "-m", &message]);
    } else {
        run_git(&["commit", "-m", &message]);
    }
    
    if sync {
        println!("Syncing (pull rebase)...");
        run_git(&["pull", "--rebase", "--reapply-cherry-picks"]);
    }

    if !nopush {
        println!("Pushing...");
        run_git(&["push"]);
    }
}

pub fn run_status_command() {
    run_status();
}

pub fn run_raw(args: Vec<String>) {
    let string_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_git(&string_args);
}
