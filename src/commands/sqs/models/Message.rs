use std::fmt;

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
