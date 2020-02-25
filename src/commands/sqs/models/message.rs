use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct RawsMessage {
  pub body: Option<String>,
  message_id: Option<String>,
  pub receipt_handle: Option<String>
}

impl RawsMessage {
  pub fn create(
    body: Option<String>,
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
      Some(txt) => write!(f, "{}", txt),
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

impl RawsSqsPolicy {
  pub fn get_source_arn(&self) -> Option<String> {
    let arn = self
      .statement[0]
      .condition.as_ref()?
      .arn_equals.as_ref()?
      .source_arn.clone();
    Some(arn)
  }
}

// {
//   "Version": "2012-10-17",
//   "Id": "arn:aws:sqs:ap-southeast-2:954088256298:rust-aws-integration/SQSDefaultPolicy",
//   "Statement": [
//     {
//       "Sid": "Sid1582345004954",
//       "Effect": "Allow",
//       "Principal": {
//         "AWS": "*"
//       },
//       "Action": "SQS:SendMessage",
//       "Resource": "arn:aws:sqs:ap-southeast-2:954088256298:rust-aws-integration",
//       "Condition": {
//         "ArnEquals": {
//           "aws:SourceArn": "arn:aws:sns:ap-southeast-2:954088256298:rust-aws-integration"
//         }
//       }
//     }
//   ]
// }
