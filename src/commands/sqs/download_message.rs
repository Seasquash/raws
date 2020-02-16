use std::error::Error;
use rusoto_sqs::*;
use super::common::construct_queue_url;
use super::models::Message::RawsMessage;
use super::list_message::retrieve_all_messages;

pub fn handler(sqs: SqsClient, queue_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let request = ReceiveMessageRequest {
    queue_url: construct_queue_url(queue_name)?,
    max_number_of_messages: Some(10),
    ..Default::default()
  };
  let messages = retrieve_all_messages(&sqs, &request, vec!());
  messages
    .iter()
    .map(|m: &RawsMessage| {
      m.receipt_handle
      .iter()
      .map(|h| {
        delete_message(&sqs, queue_name, h.into())
      })
    })
    .for_each(drop);
    
  Ok(vec!())
}

fn delete_message(sqs: &SqsClient, queue_name: &str, receipt_handle: String) -> Result<(), Box<dyn Error>> {
  let delete_message_request = DeleteMessageRequest {
      queue_url: construct_queue_url(queue_name)?,
      receipt_handle
  };

  sqs
    .delete_message(delete_message_request)
    .sync()?;

  Ok(())
}