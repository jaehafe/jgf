use jgf::cli::Cli;
use jgf::AppResult;
use clap::Parser;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = Cli::parse();
    cli.run().await
}