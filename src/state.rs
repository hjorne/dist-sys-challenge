use crate::messages::target::Node;

#[derive(Default)]
pub struct State {
    pub seen_messages: Vec<i64>,
    pub id: Node,
    pub adj_nodes: Vec<Node>,
}
