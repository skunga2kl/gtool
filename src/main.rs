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
        message: String
    },

    Raw {
        #[arg()]
        args: Vec<String>,
    }
}

fn run_git(args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git");

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Quick { message } => {
            println!("Adding...");
            run_git(&["add", "."]);

            println!("Committing...");
            run_git(&["commit", "-m", &message]);

            println!("Pushing...");
            run_git(&["push"]);
        }

        Commands::Raw { args } => {
            let string_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            run_git(&string_args);
        }
    }
}


