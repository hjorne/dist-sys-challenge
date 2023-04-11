use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Generate {
    msg_id: i64,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GenerateOk {
    in_reply_to: i64,
    id: String,
}

impl Generate {
    pub fn reply(self) -> GenerateOk {
        GenerateOk {
            in_reply_to: self.msg_id,
            id: Uuid::new_v4().to_string(),
        }
    }
}
