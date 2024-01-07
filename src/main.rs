mod cli;
mod resources;
mod utils;
use crate::resources::distro;

use crate::cli::arguments;
use crate::cli::arguments::Commands as commands;
use crate::cli::arguments::Resources as res;
use crate::resources::analyze;
use crate::utils::search;
use indicatif::ProgressBar;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = arguments::parsed_arguments();
    match args.command {
        commands::Generate {
            input_path,
            output_path,
        } => {
            let config_paths = search::paths(input_path.as_str());
            let number_of_configs_found = config_paths.len() as u64;
            let pb = ProgressBar::new(number_of_configs_found);
            for path in config_paths {
                analyze::interpret(path, &output_path).await;
                pb.inc(1)
            }
            pb.finish_and_clear();
            println!(
                "Results: doctor-crab has found {} configs and analyzed them in {:?}",
                number_of_configs_found,
                pb.elapsed()
            );
        }
        commands::Single {
            id,
            resource_type,
            output_file_name,
        } => match resource_type {
            res::Cloudfront => {
                distro::generate_markdown(&id, output_file_name).await;
            }
        },
    }

    Ok(())
}
