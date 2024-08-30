use clap::Parser;
use anyhow::Result;
use valrs::diff::{diff, DiffArgs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Diff(DiffArgs)
}

 fn main() -> Result<()> {
     let args = Cli::parse();
     match &args.command {
         Commands::Diff(cmd_args) => diff(cmd_args)?
     }
     Ok(())
}
