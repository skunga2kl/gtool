use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "gtool")]
#[command(about = "A simple Git helper tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Quick {
        #[arg(short, long)]
        message: String,

        #[arg(long, default_value_t = false)]
        nopush: bool,

        #[arg(long, default_value_t = false)]
        amend: bool,

        #[arg(short, long, default_value_t = false)]
        all: bool,

        #[arg(long)]
        files: Vec<String>
    },

    Raw {
        #[arg()]
        args: Vec<String>,
    },

    Status 
}

fn run_git(args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git");

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

fn run_status() {
    let output = Command::new("git")
        .args(&["status", "--short", "--branch"])
        .output()
        .expect("failed to run git");

    let text = String::from_utf8_lossy(&output.stdout);
    println!("{}", text);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Quick { message, nopush, amend, all, files } => {
            println!("Adding...");

            if !files.is_empty() {
                let file_refs: Vec<&str> = files.iter().map(|f| f.as_str()).collect();
                run_git(&["add"]);
                run_git(&file_refs);
            }

            else if all {
                
            } else {
                run_git(&["add", "."]);
            }

            println!("Committing...");
            if amend {
                run_git(&["commit", "--amend", "-m", &message]);
            } else if all {
                run_git(&["commit", "-a", "-m", &message]);            
            } else {
                run_git(&["commit", "-m", &message]);
            }           

            if !nopush {
                println!("Pushing...");
                run_git(&["push"]);
            }
        }

        Commands::Status => {
            run_status();
        }

        Commands::Raw { args } => {
            let string_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            run_git(&string_args);
        }
    }
}


