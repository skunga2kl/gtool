use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gtool")]
#[command(about = "A simple Git helper tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
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
        files: Vec<String>,

        #[arg(long, default_value_t = false)]
        sync: bool
    },

    Raw {
        #[arg()]
        args: Vec<String>,
    },

    Status
}
