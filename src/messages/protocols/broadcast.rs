use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Broadcast {
    msg_id: i64,
    message: i64,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BroadcastOk {
    in_reply_to: i64,
}

impl Broadcast {
    pub fn reply(self, state: &mut State) -> BroadcastOk {
        state.seen_messages.push(self.message);
        BroadcastOk {
            in_reply_to: self.msg_id,
        }
    }
}
