use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, value_name = "FILE")]
    pub output_path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Sum {
        #[arg(short, long, value_name = "FILE")]
        first_data_path: Option<PathBuf>,
        #[arg(short, long, value_name = "FILE")]
        second_data_path: Option<PathBuf>,
    },
    Nothing {},
}