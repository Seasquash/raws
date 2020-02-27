use std::fmt;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone)]
pub struct RawsMessage {
  pub body: Option<RawsSnsMessage>,
  message_id: Option<String>,
  pub receipt_handle: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RawsSnsMessage {
  pub message: String
}

impl RawsMessage {
  pub fn create(
    body: Option<RawsSnsMessage>,
    message_id: Option<String>,
    receipt_handle: Option<String>
  ) -> RawsMessage {
    RawsMessage {
      body,
      message_id,
      receipt_handle
    }
  }
}

impl fmt::Display for RawsMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.body {
      Some(txt) => write!(f, "{}", txt.message),
      None => write!(f, "")
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct RawsSqsSourceArn {
  #[serde(rename(deserialize = "aws:SourceArn"))]
  source_arn: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RawsSqsCondition {
  arn_equals: Option<RawsSqsSourceArn>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RawsSqsStatement {
  sid: String,
  action: String,
  resource: String,
  condition: Option<RawsSqsCondition>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawsSqsPolicy {
  id: String,
  statement: Vec<RawsSqsStatement>
}
