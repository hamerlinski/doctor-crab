use clap::Parser;

#[derive(Parser)]
#[command(name = "doctor-crab")]
#[command(author = "Jakub Hamerliński")]
#[command(version = "0.0.1")]
#[command(about = "doctor-crab: Generates cloud resources docs", long_about = None)]
pub struct Cli {
    /// Path to the parent directory in which doctor-crab will search for .doctor-crab.yaml files
    #[arg(short, long, default_value = "./")]
    pub input_path: String,
    /// Path to the parent directory under which documentation will be generated
    #[arg(short, long, default_value = "docs")]
    pub output_path: String,
}

pub(crate) fn parsed_arguments() -> Cli {
    return Cli::parse();
}
