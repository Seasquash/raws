use std::error::Error;
use rusoto_sqs::*;
use super::models::message::{ RawsMessage, RawsSnsMessage };
use super::common::construct_queue_url;

fn retrieve_messages(client: &SqsClient, request: &ReceiveMessageRequest) -> Result<Vec<RawsMessage>, Box<dyn Error>> {
  Ok(client
      .receive_message(request.clone())
      .sync()?
      .messages
      .unwrap_or_default()
      .iter()
      .flat_map(|message: &Message| {
          let aws_message = message.clone();
          Some(RawsMessage::create(
              Some(RawsSnsMessage { message: aws_message.body.unwrap_or_default() }),
              aws_message.message_id,
              aws_message.receipt_handle
          ))
      })
      .collect::<Vec<RawsMessage>>()
  )
}

pub fn retrieve_all_messages(client: &SqsClient, request: &ReceiveMessageRequest, result: Vec<RawsMessage>) -> Vec<RawsMessage> {
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
