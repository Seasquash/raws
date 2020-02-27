use std::error::Error;
use std::fs::File;
use rusoto_sqs::*;
use super::common::construct_queue_url;
use super::models::message::{ RawsMessage, RawsSnsMessage };
use super::list_message::retrieve_all_messages;

use crate::output::writers::write_to_file;

pub fn handler(sqs: SqsClient, queue_name: &str, to_delete: bool) -> Result<Vec<String>, Box<dyn Error>> {
  let request = ReceiveMessageRequest {
    queue_url: construct_queue_url(queue_name)?,
    max_number_of_messages: Some(10),
    ..Default::default()
  };
  let messages = retrieve_all_messages(&sqs, &request, vec!());
  // For each message, get the receipt handle, call the delete message,
  // print the message body and save it to a file
  let path = "sqs_messages.txt";
  let output = File::create(path)?;

  let result = messages
    .iter()
    .map(|m: &RawsMessage| {
      let msg = m.clone();
      match msg.receipt_handle {
        Some(receipt) => {
          let text_to_write = msg.body.unwrap_or_else(|| RawsSnsMessage {
            message: "NO MESSAGE FOUND".into()
          });
          if to_delete {
            match delete_message(&sqs, queue_name, receipt) {
              Ok(()) => {
                write_to_file(&output, &text_to_write.message);
                format!("Deleted message: {}", &text_to_write.message)
              },
              _ => {
                format!("Failed to delete message: {}", text_to_write.message)
              }
            }
          } else {
            write_to_file(&output, &text_to_write.message);
            format!("Message downloaded: {}", text_to_write.message)
          }
        },
        None => "No messages found".into()
      }
    })
    .collect::<Vec<String>>();
  Ok(result)
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