use std::error::Error;
use rusoto_sqs::*;

use super::common::construct_queue_url;

pub fn handler(sqs: SqsClient, queue_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let request = GetQueueAttributesRequest {
    queue_url: construct_queue_url(queue_name)?,
    attribute_names: Some(vec!("All".into()))
  };

  Ok(sqs
    .get_queue_attributes(request)
    .sync()?
    .attributes
    .unwrap_or_default()
    .iter()
    .map(|(k, v)| format!("{}: {}", k, v))
    .collect::<Vec<String>>()
  )
}
