use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::messages::protocols::sync_broadcast::SyncBroadcast;
use crate::messages::request::{Request, RequestBody};
use crate::messages::target::Node;
use crate::messages::target::Target;

pub struct Syncer {
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
    Topology { adj_nodes: Vec<Node> },
    Syncd { msg_id: Uuid },
    ToSync { src: Node, value: i64 },
}

impl Syncer {
    pub fn new(receiver: Receiver<SyncMsg>) -> Syncer {
        Syncer {
            receiver,
            knowledge: Default::default(),
            waiting: Default::default(),
        }
    }

    pub fn run(&mut self) {
        let mut to_sync = HashMap::new();
        while let Ok(sync_msg) = self.receiver.try_recv() {
            match &sync_msg {
                SyncMsg::Syncd { msg_id } => {
                    self.waiting.remove(msg_id);
                }
                SyncMsg::ToSync { src, dst, value } => {
                    to_sync
                        .entry((src, dst))
                        .or_insert(HashSet::new())
                        .insert(value);
                }
            }
        }

        self.resync();

        for ((src, dst), values) in to_sync {
            self.send(src, dst, values);
        }
    }

    fn send(&mut self, src: Node, dest: Node, values: HashSet<i64>) {
        let src = Target::Node(src);
        let dest = Target::Node(dest);
        let uuid = Uuid::new_v4();
        let req = Request {
            src,
            dest,
            body: RequestBody::SyncBroadcast(SyncBroadcast {
                msg_id: uuid,
                messages: values.clone(),
            }),
        };
        println!("{}", &serde_json::to_string(&req).unwrap());
        self.waiting.insert(uuid, (src, dest, values));
    }

    fn resync(&self) {
        for (id, (src, dest, values)) in &self.waiting {
            let req = Request {
                src: *src,
                dest: *dest,
                body: RequestBody::SyncBroadcast(SyncBroadcast {
                    msg_id: *id,
                    messages: values.clone(),
                }),
            };
            println!("{}", &serde_json::to_string(&req).unwrap());
        }
    }
}
