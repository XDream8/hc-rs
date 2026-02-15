use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Remove duplicate lines from final/output file
    #[arg(short = 'r', long)]
    pub remove_duplicate_lines: bool,

    /// Output file name
    #[arg(short = 'o', long, default_value = "hosts")]
    pub output_file: String,

    /// Ignore fetching errors
    #[arg(short = 'i', long)]
    pub ignore_fetching_errors: bool,

    /// Host URLs or file paths
    #[arg(value_name = "URLS")]
    pub urls: Vec<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, PartialEq)]
pub enum Commands {
    /// Remove duplicate lines from file(s)
    #[command(arg_required_else_help = true)]
    #[clap(name = "rmd")]
    Rmd {
        /// file(s) to remove duplicates from
        #[arg(value_name = "FILE")]
        files: Vec<PathBuf>,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CliArgs::command().debug_assert();
}
