mod resources;

use crate::resources::distro;
use aws_sdk_cloudfront as cloudfront;

#[tokio::main]
async fn main() -> Result<(), cloudfront::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_cloudfront::Client::new(&config);

    let distro_data = client
        .get_distribution()
        .id("XYZ")
        .send()
        .await
        .unwrap();
    let distro_output = distro_data.distribution.unwrap();
    distro::generate_markdown(distro_output);

    Ok(())
}
