use uuid::Uuid;

use crate::message::{Node, Request, RequestBody, Response, ResponseBody};

pub struct Handler {
    seen_messages: Vec<i64>,
    id: Node,
    adj_nodes: Vec<Node>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            seen_messages: Vec::new(),
            id: Node(-1),
            adj_nodes: Vec::new(),
        }
    }

    pub fn handle(&mut self, msg: Request) -> Response {
        match msg.body {
            RequestBody::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                self.id = node_id;
                self.adj_nodes = node_ids;
                Response {
                    src: msg.dest,
                    dest: msg.src,
                    body: ResponseBody::InitOk {
                        in_reply_to: msg_id,
                    },
                }
            }

            RequestBody::Echo { msg_id, echo } => Response {
                src: msg.dest,
                dest: msg.src,
                body: ResponseBody::EchoOk {
                    in_reply_to: msg_id,
                    echo,
                },
            },

            RequestBody::Generate { msg_id } => Response {
                src: msg.dest,
                dest: msg.src,
                body: ResponseBody::GenerateOk {
                    in_reply_to: msg_id,
                    id: Uuid::new_v4().to_string(),
                },
            },

            RequestBody::Broadcast { msg_id, message } => {
                self.seen_messages.push(message);
                Response {
                    src: msg.dest,
                    dest: msg.src,
                    body: ResponseBody::BroadcastOk {
                        in_reply_to: msg_id,
                    },
                }
            }

            RequestBody::Read { msg_id } => Response {
                src: msg.dest,
                dest: msg.src,
                body: ResponseBody::ReadOk {
                    in_reply_to: msg_id,
                    messages: self.seen_messages.clone(),
                },
            },

            RequestBody::Topology {
                msg_id,
                mut topology,
            } => {
                self.adj_nodes = topology.remove(&self.id).unwrap();
                Response {
                    src: msg.dest,
                    dest: msg.src,
                    body: ResponseBody::TopologyOk {
                        in_reply_to: msg_id,
                    },
                }
            }
        }
    }
}
