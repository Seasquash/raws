use std::env;
use std::error::Error;

pub fn construct_queue_url(queue_name: &str) -> Result<String, Box<dyn Error>> {
  Ok(String::from(format!(
      "https://sqs.{region}.amazonaws.com/{account}/{queue_name}",
      region = env::var("AWS_DEFAULT_REGION").expect("AWS REGION NOT FOUND"),
      account = env::var("AWS_ACCOUNT").expect("AWS ACCOUNT NOT FOUND"),
      queue_name = queue_name
  )))
}
