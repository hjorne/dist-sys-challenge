use crate::messages::target::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::state::State;
use crate::syncer::SyncMsg;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SyncBroadcast {
    pub msg_id: Uuid,
    pub messages: HashSet<i64>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SyncBroadcastOk {
    pub in_reply_to: Uuid,
}

impl SyncBroadcast {
    pub fn reply(self, src: Target, state: &mut State) -> SyncBroadcastOk {
        let node = match src {
            Target::Node(node) => Some(node),
            _ => None,
        };
        for msg in &self.messages {
            state
                .sender
                .send(SyncMsg::ToSync {
                    src: node,
                    value: *msg,
                })
                .unwrap();
        }
        state.seen_messages.extend(self.messages);
        SyncBroadcastOk {
            in_reply_to: self.msg_id,
        }
    }
}
