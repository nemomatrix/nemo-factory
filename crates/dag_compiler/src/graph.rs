use std::collections::HashMap;
use crate::node::Node;

pub struct DAG {
    pub nodes: HashMap<String, Node>,
}

impl DAG {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn get_ready_nodes(&self) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| n.dependencies.is_empty())
            .collect()
    }
}
