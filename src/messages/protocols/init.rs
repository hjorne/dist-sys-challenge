use serde::{Deserialize, Serialize};

use crate::messages::target::Node;
use crate::state::State;
use crate::syncer::SyncMsg;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Init {
    pub msg_id: i64,
    pub node_id: Node,
    pub node_ids: Vec<Node>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InitOk {
    in_reply_to: i64,
}

impl Init {
    pub fn reply(self, state: &mut State) -> InitOk {
        state.id = self.node_id;
        state.adj_nodes = self
            .node_ids
            .into_iter()
            .filter(|p| p.0 != self.node_id.0)
            .collect();
        state
            .sender
            .send(SyncMsg::Topology {
                adj_nodes: state.adj_nodes.clone(),
            })
            .unwrap();
        InitOk {
            in_reply_to: self.msg_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::protocols::init::{Init, InitOk};
    use crate::messages::target::Node;

    #[test]
    fn init_deserialization() {
        let json = "{\"type\":\"Init\",\"node_id\":\"n0\",\"node_ids\":[\"n0\"],\"msg_id\":1}";
        let init = serde_json::from_str::<Init>(json).unwrap();
        assert_eq!(
            init,
            Init {
                msg_id: 1,
                node_id: Node(0),
                node_ids: vec![Node(0)]
            }
        )
    }

    #[test]
    fn init_ok_serialization() {
        let init_ok = InitOk { in_reply_to: 20 };
        assert_eq!(
            "{\"in_reply_to\":20}",
            serde_json::to_string(&init_ok).unwrap()
        );
    }
}
