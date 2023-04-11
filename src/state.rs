use std::collections::HashSet;
use std::sync::mpsc::Sender;

use crate::messages::target::Node;
use crate::syncer::SyncMsg;

pub struct State {
    pub id: Node,
    pub adj_nodes: Vec<Node>,
    pub seen_messages: HashSet<i64>,
    pub sender: Sender<SyncMsg>,
}

impl State {
    pub fn new(sender: Sender<SyncMsg>) -> State {
        State {
            sender,
            id: Default::default(),
            adj_nodes: Default::default(),
            seen_messages: Default::default(),
        }
    }
}
