use crate::state::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Read {
    msg_id: i64,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadOk {
    in_reply_to: i64,
    messages: Vec<i64>,
}

impl Read {
    pub fn reply(self, state: &State) -> ReadOk {
        ReadOk {
            in_reply_to: self.msg_id,
            messages: state.seen_messages.clone().into_iter().collect(),
        }
    }
}
