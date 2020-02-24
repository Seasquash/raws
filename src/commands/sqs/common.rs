use std::env;
use std::error::Error;
use std::collections::HashMap;

use super::models::message::RawsSqsPolicy;

pub fn construct_queue_url(queue_name: &str) -> Result<String, Box<dyn Error>> {
  Ok(format!(
    "https://sqs.{region}.amazonaws.com/{account}/{queue_name}",
    region = env::var("AWS_DEFAULT_REGION").expect("AWS REGION NOT FOUND"),
    account = env::var("AWS_ACCOUNT").expect("AWS ACCOUNT NOT FOUND"),
    queue_name = queue_name
  ))
}

pub fn get_sqs_policy(attributes: HashMap<String, String>) -> Option<RawsSqsPolicy> {
  if let Some(p) = attributes.get("Policy") {
    Some(serde_json::from_str(&p).unwrap())
  } else {
    None
  }
}
