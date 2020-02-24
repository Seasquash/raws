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
pub struct RawsSqsSourceArn {
  #[serde(rename(deserialize = "aws:SourceArn"))]
  pub source_arn: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawsSqsCondition {
  pub arn_equals: RawsSqsSourceArn
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawsSqsStatement {
  sid: String,
  action: String,
  resource: String,
  pub condition: RawsSqsCondition
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawsSqsPolicy {
  id: String,
  pub statement: Vec<RawsSqsStatement>
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