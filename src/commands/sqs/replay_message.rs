use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::error::Error;
use rusoto_sqs::*;
use uuid::Uuid;

use super::common::{ construct_queue_url };

pub fn handler(sqs: SqsClient, queue_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let request = ListDeadLetterSourceQueuesRequest {
    queue_url: construct_queue_url(queue_name)?,
  };

  let queue_urls = sqs
    . list_dead_letter_source_queues(request)
    .sync()?
    .queue_urls;

  if let Some(queue_url) = queue_urls.first() {
    let messages_or = read_messages_from_file("./sqs_messages.txt".into());

    match messages_or {
      Ok(entries) => {
        let result = sqs
          .send_message_batch(SendMessageBatchRequest {
            entries,
            queue_url: queue_url.into()
          })
          .sync()?;

        Ok(vec!(
          format!("Successfully replayed {} messages", result.successful.len()),
          format!("Fail to replay {} messages", result.failed.len())
        ))
      },
      Err(_) => Ok(vec!("Failed to read messages from file".into()))
    }
  } else {
    Ok(vec!())
  }

}

fn read_messages_from_file(path: String) -> Result<Vec<SendMessageBatchRequestEntry>, Box<dyn Error>> {
  let input = File::open(path)?;
  let buffered = BufReader::new(input);

  Ok(buffered
    .lines()
    .flat_map(|body| body
      .map(|message_body| SendMessageBatchRequestEntry {
        id:  Uuid::new_v4().to_string(),
        message_body,
        ..Default::default()
      })
    )
    .collect::<Vec<SendMessageBatchRequestEntry>>()
  )
}
