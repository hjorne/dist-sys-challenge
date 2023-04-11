use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::messages::target::Node;
use crate::state::State;
use crate::syncer::SyncMsg;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Topology {
    pub msg_id: i64,
    pub topology: HashMap<Node, Vec<Node>>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TopologyOk {
    in_reply_to: i64,
}

impl Topology {
    pub fn reply(mut self, state: &mut State) -> TopologyOk {
        state.adj_nodes = self
            .topology
            .remove(&state.id)
            .unwrap()
            .into_iter()
            .filter(|p| p.0 != state.id.0)
            .collect();
        state
            .sender
            .send(SyncMsg::Topology {
                adj_nodes: state.adj_nodes.clone(),
            })
            .unwrap();
        TopologyOk {
            in_reply_to: self.msg_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::protocols::topology::Topology;
    use crate::messages::request::RequestBody;
    use crate::messages::target::Node;
    use std::collections::HashMap;

    #[test]
    fn serialization() {
        let json = "{\"type\":\"topology\",\"topology\":{\"n0\":[]},\"msg_id\":1}";
        let topology = serde_json::from_str::<RequestBody>(json).unwrap();
        assert_eq!(
            topology,
            RequestBody::Topology(Topology {
                msg_id: 1,
                topology: {
                    let mut map = HashMap::new();
                    map.insert(Node(0), Vec::new());
                    map
                }
            })
        )
    }
}
