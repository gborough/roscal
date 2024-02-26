mod cli;
mod models;

use clap::Parser;
use cli::{
    cli_fn::{run_dissect, run_merge, run_validate, show_dissect},
    cli_opts::{Commands, OscalCli},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = OscalCli::parse();

    match cli.command {
        Commands::Dissect(ref opts) => run_dissect(opts).await?,
        Commands::Merge(ref opts) => run_merge(opts).await?,
        Commands::Validate(ref opts) => run_validate(opts).await?,
        Commands::ShowDissect => show_dissect().await?,
    }

    Ok(())
}
