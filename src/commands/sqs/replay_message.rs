use std::error::Error;
use rusoto_sqs::*;

use super::common::{ construct_queue_url, get_sqs_policy };

pub fn handler(sqs: SqsClient, queue_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let request = GetQueueAttributesRequest {
    queue_url: construct_queue_url(queue_name)?,
    attribute_names: Some(vec!("Policy".into()))
  };

  let attributes = sqs
    .get_queue_attributes(request)
    .sync()?
    .attributes
    .unwrap_or_default();
  
  if let Some(policy) = get_sqs_policy(attributes) {
    Ok(vec!(policy.statement[0].condition.arn_equals.source_arn.clone()))
  } else {
    Ok(vec!())
  }

}
