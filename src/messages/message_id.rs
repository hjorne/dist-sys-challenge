use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageId {
    Int(i64),
    Uuid(Uuid),
}
