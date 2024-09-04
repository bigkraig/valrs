use clap::Parser;
use anyhow::Result;
use valrs::diff::{diff, DiffArgs};
use valrs::dump::{dump, DumpArgs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Diff(DiffArgs),
    Dump(DumpArgs),
}

 fn main() -> Result<()> {
     let args = Cli::parse();
     match &args.command {
         Commands::Diff(cmd_args) => diff(cmd_args)?,
         Commands::Dump(cmd_args) => dump(cmd_args)?
     }
     Ok(())
}
