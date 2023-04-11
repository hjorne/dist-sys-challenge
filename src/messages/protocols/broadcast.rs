use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Broadcast {
    pub msg_id: i64,
    pub message: i64,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BroadcastOk {
    pub in_reply_to: i64,
}

impl Broadcast {
    pub fn reply(self, state: &mut State) -> BroadcastOk {
        state.seen_messages.insert(self.message);

        for dest in &state.adj_nodes {
            state.syncer.send(state.id, *dest, self.message);
        }

        BroadcastOk {
            in_reply_to: self.msg_id,
        }
    }
}
