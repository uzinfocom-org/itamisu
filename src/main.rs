use clap::Parser;
use itamisu::error::pretty_exit;
use itamisu::{scheme::Itamisu, Cli, Commands, Result};

static INCLUDED: &str = include_str!("../foods.toml");

fn run() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Included { times } => Itamisu::builder()
            .text(INCLUDED)
            .and_then(|i| i.build())
            .and_then(|itms| itms.random(times))
            .and_then(|itms| itms.show()),
        Commands::Custom { times, path } => Itamisu::builder()
            .read(path)
            .and_then(|i| i.build())
            .and_then(|itms| itms.random(times))
            .and_then(|itms| itms.show()),
        Commands::New { path } => Itamisu::builder()
            .text(INCLUDED)
            .and_then(|i| i.build())
            .and_then(|itms| itms.write(path)),
    }
}

fn main() -> Result<()> {
    run().unwrap_or_else(|e| pretty_exit(e.to_string()));
    Ok(())
}
