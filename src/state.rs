use crate::messages::target::Node;
use crate::syncer::Syncer;
use std::collections::HashSet;

#[derive(Default)]
pub struct State {
    pub seen_messages: HashSet<i64>,
    pub id: Node,
    pub adj_nodes: Vec<Node>,
    pub syncer: Syncer,
}
