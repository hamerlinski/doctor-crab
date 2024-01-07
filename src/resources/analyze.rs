use crate::resources::distro;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

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
pub(crate) async fn interpret(path: String, output_path: &String) {
    let mut yaml_content = String::new();
    File::open(path)
        .expect("Could not find file")
        .read_to_string(&mut yaml_content)
        .expect("Could not read file");
    let config: Config = serde_yaml::from_str(&yaml_content).expect("Could not open file.");
    for (_env_name, env) in &config.environments {
        distro::generate_markdown(
            &env.resources.cloudfront,
            Some(format!(
                "{output_root}/{app}/{env}/",
                output_root = output_path,
                app = config.app,
                env = _env_name
            )),
        )
        .await;
    }
}
