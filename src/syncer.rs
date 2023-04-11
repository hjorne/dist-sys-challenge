use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::messages::protocols::sync_broadcast::SyncBroadcast;
use crate::messages::request::{Request, RequestBody};
use crate::messages::target::Node;
use crate::messages::target::Target;

pub struct Syncer {
    count: i64,
    waiting: HashMap<Uuid, (Target, Target, HashSet<i64>)>,
    receiver: Receiver<SyncMsg>,
}

#[derive(Copy, Clone)]
pub enum SyncMsg {
    Syncd { msg_id: Uuid },
    ToSync { src: Node, dst: Node, value: i64 },
}

impl Syncer {
    pub fn new(receiver: Receiver<SyncMsg>) -> Syncer {
        Syncer {
            receiver,
            count: 0,
            waiting: Default::default(),
        }
    }

    pub fn run(&mut self) {
        let mut to_sync = HashMap::new();
        while let Ok(sync_msg) = self.receiver.try_recv() {
            self.count += 1;
            match sync_msg {
                SyncMsg::Syncd { msg_id } => {
                    eprintln!("Received confirmation on {msg_id}");
                    self.waiting.remove(&msg_id);
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
        eprintln!("Sync to {:?}:{} on {:?}", dest, uuid, &values);
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
            eprintln!("Resync to {:?}:{} on {:?}", dest, id, &values);
            println!("{}", &serde_json::to_string(&req).unwrap());
        }
    }
}
