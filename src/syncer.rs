use crate::messages::protocols::broadcast::Broadcast;
use crate::messages::request::{Request, RequestBody};
use crate::messages::target::Node;
use crate::messages::target::Target;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Syncer {
    id: i64,
    to_sync: HashMap<i64, (Target, Target, i64)>,
}

impl Syncer {
    pub fn send(&mut self, src: Node, dest: Node, i: i64) {
        dbg!(&self);
        let src = Target::Node(src);
        let dest = Target::Node(dest);
        let req = Request {
            src,
            dest,
            body: RequestBody::Broadcast(Broadcast {
                msg_id: self.id,
                message: i,
            }),
        };
        println!("{}", &serde_json::to_string(&req).unwrap());
        self.to_sync.insert(self.id, (src, dest, i));
        self.id += 1;

        if self.id % 20 == 0 {
            self.resync();
        }
    }

    fn resync(&mut self) {
        for (src, dest, i) in self.to_sync.values() {
            let req = Request {
                src: *src,
                dest: *dest,
                body: RequestBody::Broadcast(Broadcast {
                    msg_id: self.id,
                    message: *i,
                }),
            };
            println!("{}", &serde_json::to_string(&req).unwrap());
        }
    }

    pub fn receive(&mut self, i: i64) {
        self.to_sync.remove(&i);
    }
}
