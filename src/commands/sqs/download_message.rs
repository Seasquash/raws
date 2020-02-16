use std::error::Error;
use std::fs::File;
use std::io::Write;
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
  // For each message, get the receipt handle, call the delete message,
  // print the message body and save it to a file
  let path = "sqs_messages.txt";
  let mut output = File::create(path)?;

  messages
    .iter()
    .map(|m: &RawsMessage| {
      let msg = m.clone();
      match msg.receipt_handle {
        Some(receipt) => {
          let deleted = delete_message(&sqs, queue_name, receipt.into());
          match deleted {
            Ok(()) => {
              let text_to_write = msg.body.unwrap_or("NO MESSAGE FOUND".into());
              write!(output, "{}\n\n", text_to_write);
              println!("Deleted message: {}", text_to_write);
            },
            _ => {
              println!("Failed to delete message: {}", msg.body.unwrap_or("NO MESSAGE FOUND".into()));
            }
          }
        },
        None => ()
      }
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