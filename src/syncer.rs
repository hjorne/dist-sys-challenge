use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::messages::protocols::sync_broadcast::SyncBroadcast;
use crate::messages::request::{Request, RequestBody};
use crate::messages::target::Node;
use crate::messages::target::Target;

pub struct Syncer {
    adj_nodes: Vec<Node>,
    id: Node,
    waiting: HashMap<Uuid, Waiting>,
    knowledge: HashMap<Node, HashSet<i64>>,
    receiver: Receiver<SyncMsg>,
}

struct Waiting {
    src: Node,
    dst: Node,
    values: HashSet<i64>,
}

#[derive(Clone)]
pub enum SyncMsg {
    Topology { id: Node, adj_nodes: Vec<Node> },
    Syncd { msg_id: Uuid },
    ToSync { src: Option<Node>, value: i64 },
}

impl Syncer {
    pub fn new(receiver: Receiver<SyncMsg>) -> Syncer {
        Syncer {
            receiver,
            adj_nodes: Default::default(),
            knowledge: Default::default(),
            waiting: Default::default(),
            id: Default::default(),
        }
    }

    pub fn run(&mut self) {
        let mut to_sync = HashMap::<Node, HashSet<i64>>::new();
        while let Ok(sync_msg) = self.receiver.try_recv() {
            match &sync_msg {
                SyncMsg::Syncd { msg_id } => {
                    if let Some(waiting) = self.waiting.remove(msg_id) {
                        for value in waiting.values {
                            self.knowledge.entry(waiting.dst).or_default().insert(value);
                        }
                    }
                }
                SyncMsg::ToSync { src, value } => {
                    if let Some(node) = src {
                        self.knowledge.entry(*node).or_default().insert(*value);
                    }

                    for node in &self.adj_nodes {
                        if !self
                            .knowledge
                            .get(node)
                            .unwrap_or(&HashSet::new())
                            .contains(value)
                        {
                            to_sync.entry(*node).or_default().insert(*value);
                        }
                    }
                }
                SyncMsg::Topology { id, adj_nodes } => {
                    self.id = *id;
                    self.adj_nodes = adj_nodes.clone();
                }
            }
        }

        self.resync();

        for (dst, values) in to_sync {
            self.send(self.id, dst, values);
        }
    }

    fn send(&mut self, src: Node, dst: Node, values: HashSet<i64>) {
        let waiting = Waiting {
            src,
            dst,
            values: values.clone(),
        };
        let src = Target::Node(src);
        let dest = Target::Node(dst);
        let uuid = Uuid::new_v4();
        let req = Request {
            src,
            dest,
            body: RequestBody::SyncBroadcast(SyncBroadcast {
                msg_id: uuid,
                messages: values,
            }),
        };
        println!("{}", &serde_json::to_string(&req).unwrap());
        self.waiting.insert(uuid, waiting);
    }

    fn resync(&self) {
        for (id, waiting) in &self.waiting {
            let req = Request {
                src: Target::Node(waiting.src),
                dest: Target::Node(waiting.dst),
                body: RequestBody::SyncBroadcast(SyncBroadcast {
                    msg_id: *id,
                    messages: waiting.values.clone(),
                }),
            };
            println!("{}", &serde_json::to_string(&req).unwrap());
        }
    }
}
