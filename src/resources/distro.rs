use aws_sdk_cloudfront::types::Distribution;
use chrono::{DateTime, Utc};
use markdown_gen::markdown::{AsMarkdown, List, Markdown};
use std::fs::File;

pub(crate) fn generate_markdown(distribution: Distribution) {
    let distribution_config = distribution.distribution_config.unwrap();
    let file = File::create(format!("{}.md", distribution_config.comment).as_str()).unwrap();
    let mut md = Markdown::new(file);
    let utc: DateTime<Utc> = Utc::now();
    md.write(format!("Cloudfront {}", distribution_config.comment).heading(1))
        .unwrap();
    md.write(
        format!(
            "<sub>Documentation last update: {} UTC</sub>",
            utc.format("%Y/%m/%d %H:%M:%S").to_string()
        )
        .as_str(),
    )
    .unwrap();
    md.write("Parameters:".heading(2)).unwrap();
    md.write(
        List::new(false)
            .item(format!("id: {}", distribution.id).as_str())
            .item(format!("domain_name: {}", distribution.domain_name).as_str())
            .item(format!("status: {}", distribution.status).as_str())
            .item(format!("arn: {}", distribution.arn).as_str())
            .item(format!("last_modified_time: {}", distribution.last_modified_time).as_str())
            .item(format!("web_acl_id: {}", distribution_config.web_acl_id.unwrap()).as_str()), //TODO: add more fields
    )
    .unwrap();
}
