use rusoto_core::RusotoError;
use rusoto_sqs::*;

pub fn handler(sqs: SqsClient) -> Result<Vec<String>, RusotoError<ListQueuesError>> {
  let request = ListQueuesRequest::default();
  Ok(sqs
      .list_queues(request)
      .sync()?
      .queue_urls
      .unwrap_or_default()
      .iter()
      .map(|url| url.split("/").last().map(|x| x.into()))
      .filter_map(|m| m)
      .collect::<Vec<String>>()
  )
}