use serde::{Deserialize, Serialize};

use crate::messages::request::Request;
use crate::messages::response::Response;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
}

#[cfg(test)]
mod tests {
    use crate::messages::message::Message;
    use crate::messages::protocols::init::Init;
    use crate::messages::protocols::topology::Topology;
    use crate::messages::request::{Request, RequestBody};
    use crate::messages::target::{Client, Node, Target};
    use std::collections::HashMap;

    #[test]
    fn message_serialization() {
        let json = "{\"id\":0,\"src\":\"c0\",\"dest\":\"n0\",\"body\":{\"type\":\"init\",\"node_id\":\"n0\",\"node_ids\":[\"n0\"],\"msg_id\":1}}\n";
        let request = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(
            request,
            Message::Request(Request {
                src: Target::Client(Client(0)),
                dest: Target::Node(Node(0)),
                body: RequestBody::Init(Init {
                    msg_id: 1,
                    node_id: Node(0),
                    node_ids: vec![Node(0)],
                }),
            })
        );
    }

    #[test]
    fn topology_serialization() {
        let json = "{\"id\":2,\"src\":\"c1\",\"dest\":\"n0\",\"body\":{\"type\":\"topology\",\"topology\":{\"n0\":[]},\"msg_id\":1}}";

        let request = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(
            request,
            Message::Request(Request {
                src: Target::Client(Client(1)),
                dest: Target::Node(Node(0)),
                body: RequestBody::Topology(Topology {
                    msg_id: 1,
                    topology: {
                        let mut map = HashMap::new();
                        map.insert(Node(0), vec![]);
                        map
                    }
                }),
            })
        );
    }
}
