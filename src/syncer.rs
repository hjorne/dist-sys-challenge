use crate::messages::protocols::broadcast::Broadcast;
use crate::messages::request::{Request, RequestBody};
use crate::messages::target::Node;
use crate::messages::target::Target;
use clokwerk::{Scheduler, TimeUnits};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Syncer {
    id: i64,
    to_sync: HashMap<i64, (Target, Target, i64)>,
}

impl Syncer {
    pub fn new() -> &'static Mutex<Syncer> {
        let scheduler = Scheduler::new();
        let mut syncher = Mutex::new(Syncer {
            ..Default::default()
        });
        scheduler
            .every(10.minutes())
            .run(|| syncher.lock().unwrap().resync());
        &syncher
    }

    pub fn send(&mut self, src: Node, dest: Node, i: i64) {
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

        if self.id % 10 == 0 {
            self.resync();
        }
    }

    fn resync(&self) {
        dbg!(&self.to_sync.len());
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
