mod cli;
mod git;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{run_quick, run_status_command, run_raw};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Quick { message, nopush, amend, all, files, sync   } =>
            run_quick(message, nopush, amend, all, files, sync),

        Commands::Status =>
            run_status_command(),

        Commands::Raw { args } =>
            run_raw(args),
    }
}
