mod resources;
mod utils;

use crate::resources::distro;
use crate::utils::search;
use aws_sdk_cloudfront as cloudfront;
use serde::{Deserialize, Serialize};
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
            let creds = aws_config::load_from_env().await;
            let client = aws_sdk_cloudfront::Client::new(&creds);
            let distro_data = client
                .get_distribution()
                .id(&env.resources.cloudfront)
                .send()
                .await
                .unwrap();
            let distro_output = distro_data.distribution.unwrap();
            distro::generate_markdown(distro_output);
        }
    }

    Ok(())
}
#[derive(Debug, Deserialize)]
struct Resources {
    cloudfront: String,
    // Add more resource fields as needed (e.g., cloudfront, alb, cluster)
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
