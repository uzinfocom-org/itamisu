#![allow(clippy::bind_instead_of_map)]

pub mod error;
pub mod scheme;

use clap::{Parser, Subcommand};
pub use error::{ItamisuError, Result};
use std::path::PathBuf;

/// A random food picker out of desperation
#[derive(Debug, Parser)]
#[command(name = "itamisu")]
#[command(about = "A random food picker out of desperation")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Use data included with binary
    Included {
        /// Amount of random repetitions needed
        #[clap(default_value_t = 1)]
        times: u8,
    },

    /// Use data included with binary
    Custom {
        /// Amount of random repetitions needed
        #[clap(default_value_t = 1)]
        times: u8,

        /// Path to the custom list of foods
        #[arg(required = true)]
        path: PathBuf,
    },

    /// Generate an example data file
    New {
        /// Where shall we save it (must end with .toml or we will append)
        #[arg(env = "PWD")]
        path: PathBuf,
    },
}
