use std::error::Error;
use std::env;
use rusoto_sqs::*;
use super::models::Message::RawsMessage;

fn construct_queue_url(queue_name: &str) -> Result<String, Box<dyn Error>> {
  Ok(String::from(format!(
      "https://sqs.{region}.amazonaws.com/{account}/{queue_name}",
      region = env::var("AWS_DEFAULT_REGION").expect("AWS REGION NOT FOUND"),
      account = env::var("AWS_ACCOUNT").expect("AWS ACCOUNT NOT FOUND"),
      queue_name = queue_name
  )))
}

fn retrieve_messages(client: &SqsClient, request: &ReceiveMessageRequest) -> Result<Vec<RawsMessage>, Box<dyn Error>> {
  Ok(client
      .receive_message(request.clone())
      .sync()?
      .messages
      .unwrap_or_default()
      .iter()
      .filter_map(|message: &Message| {
          let aws_message = message.clone();
          Some(RawsMessage::create(
              aws_message.body,
              aws_message.message_id,
              aws_message.receipt_handle
          ))
      })
      .collect::<Vec<RawsMessage>>()
  )
}

fn retrieve_all_messages(client: &SqsClient, request: &ReceiveMessageRequest, result: Vec<RawsMessage>) -> Vec<RawsMessage> {
  let msgs = retrieve_messages(client, request);
  // if Err or Vec is empty, return result
  // otherwise, call retrieve_all_messages passing results + Vec
  match msgs {
      Ok(v) => {
          if v.is_empty() {
              result
          } else {
              let new_result = result.into_iter().chain(v.into_iter()).collect();
              retrieve_all_messages(&client, &request, new_result)
          }
      },
      Err(e) => {
          println!("An error occurred: {}", e);
          result
      }
  }
}

// Example: "https://sqs.ap-southeast-2.amazonaws.com/954088256298/rust-aws-integration"
pub fn handler(sqs: SqsClient, queue_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let request = ReceiveMessageRequest {
      queue_url: construct_queue_url(queue_name)?,
      max_number_of_messages: Some(10),
      ..Default::default()
  };
  let messages = retrieve_all_messages(&sqs, &request, vec!())
      .iter()
      .map(|msg| format!("{}", msg))
      .collect::<Vec<String>>();
  Ok(messages)
}
