mod cli;
mod resources;
mod utils;

use crate::cli::arguments;
use crate::resources::analyze;
use crate::utils::search;
use std::io;
#[tokio::main]
async fn main() -> io::Result<()> {
    let arg = arguments::parsed_arguments();
    let config_paths = search::paths(arg.input_path.as_str());
    let number_of_configs_found = config_paths.len() as u64;
    for path in config_paths {
        analyze::interpret(path, &arg).await;
    }

    Ok(())
}
