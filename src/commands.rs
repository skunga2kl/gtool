use crate::git::{run_git, run_status};
use colored::Colorize;

pub fn run_quick (
    message: String,
    nopush: bool,
    amend: bool,
    all: bool,
    files: Vec<String>,
    sync: bool
) {
    if message.trim().is_empty() {
        eprintln!("{}", "Error: you cannot have an empty commit message".red());
        return;
    }

    if sync {
        println!("{}", "Syncing (pull rebase)...".green());
        println!(
            "{}",
            "Note: if you have local changes, the rebase may cause conflicts"
                .yellow()
        );
        run_git(&["pull", "--rebase"]);
    }    

    println!("{}", "Adding...".bright_green());

    if !files.is_empty() {
        let mut cmd: Vec<&str> = vec!["add"];
        for f in &files {
            cmd.push(f.as_str());
        }
        run_git(&cmd);    
    } else if all {
        run_git(&["add", "-A"]);
    } else {
        run_git(&["add", "."]);
    }

    println!("{}", "Committing...".bright_green());
    if amend {
        run_git(&["commit", "--amend", "-m", &message]);
    } else if all {
        run_git(&["commit", "-a", "-m", &message]);
    } else {
        run_git(&["commit", "-m", &message]);
    }
    
    if sync {
        println!("{}", "Syncing (pull rebase)...".bright_green());
        run_git(&["pull", "--rebase", "--reapply-cherry-picks"]);

        println!(
            "{}",
            "Note: if you see conflict markers in files, the rebase needs manual fixing"
            .yellow()
        )
    }

    if !nopush {
        println!("{}", "Pushing...".bright_green());
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
