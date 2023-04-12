use crate::messages::target::Target;
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
    pub fn reply(self, src: Target, state: &mut State) -> BroadcastOk {
        state
            .sender
            .send(SyncMsg::ToSync {
                src: match src {
                    Target::Node(node) => Some(node),
                    _ => None,
                },
                value: self.message,
            })
            .unwrap();
        state.seen_messages.insert(self.message);
        BroadcastOk {
            in_reply_to: self.msg_id,
        }
    }
}
