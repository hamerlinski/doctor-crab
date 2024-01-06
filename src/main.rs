mod resources;
mod utils;

use crate::resources::distro;
use crate::utils::search;
use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::io;
use std::io::Read;

#[tokio::main]
async fn main() -> io::Result<()> {
    for path in search::paths("./test") {
        let mut yaml_content = String::new();
        File::open(path)?.read_to_string(&mut yaml_content)?;
        let config: Config = serde_yaml::from_str(&yaml_content).expect("Could not open file.");
        for (_env_name, env) in &config.environments {
            distro::generate_markdown(
                &env.resources.cloudfront,
                Some(format!("docs/{}/{}/", config.app, _env_name)),
            )
            .await;
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Resources {
    cloudfront: String,
}

#[derive(Debug, Deserialize)]
struct Env {
    resources: Resources,
}

#[derive(Debug, Deserialize)]
struct Config {
    app: String,
    #[serde(flatten)]
    environments: std::collections::BTreeMap<String, Env>,
}
