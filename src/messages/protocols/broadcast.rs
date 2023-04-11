use serde::{Deserialize, Serialize};

use crate::state::State;
use crate::syncer::SyncMsg;

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
        eprintln!("Broadcast {}", self.message);
        state.seen_messages.insert(self.message);
        for node in &state.adj_nodes {
            state
                .sender
                .send(SyncMsg::ToSync {
                    src: state.id,
                    dst: *node,
                    value: self.message,
                })
                .unwrap();
        }
        BroadcastOk {
            in_reply_to: self.msg_id,
        }
    }
}
