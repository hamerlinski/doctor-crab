use clap::{crate_version, Parser, Subcommand, ValueEnum};
#[derive(Parser)]
#[command(name = "doctor-crab")]
#[command(author = "Jakub Hamerliński")]
#[command(version = crate_version!())]
#[command(about = "doctor-crab: Generates cloud resources docs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generates documentation for provided resources found in .doctor-crab.yaml files
    #[command(arg_required_else_help = true)]
    Generate {
        /// Path to the parent directory in which doctor-crab will search for .doctor-crab.yaml files
        #[arg(short, long, default_value = "./")]
        input_path: String,
        /// Path to the parent directory in which documentation will be generated
        #[arg(short, long, default_value = "docs")]
        output_path: String,
    },
    /// Generates single markdown documentation file for resource with provided id
    #[command(arg_required_else_help = true)]
    Single {
        /// Type of the resource
        #[arg(short, long)]
        resource_type: Resources,
        /// Id of the resource that doctor-crab should search for
        #[arg(short, long)]
        id: String,
        /// File name where information about resource should be saved
        #[arg(short, long, required = false, default_value = "./")]
        output_file_name: Option<String>,
    },
}
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Resources {
    Cloudfront,
}
pub(crate) fn parsed_arguments() -> Cli {
    return Cli::parse();
}
