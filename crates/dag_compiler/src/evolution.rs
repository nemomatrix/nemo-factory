use crate::graph::DAG;
use crate::node::Node;
use crate::scorer::Scorer;

pub struct Evolver;

impl Evolver {
    pub fn evolve(dag: &mut DAG, ci_result: bool) {
        for node in dag.nodes.values_mut() {
            let score = Scorer::score(node, ci_result, 120);

            node.stability_score = score;

            if score < 0.3 {
                node.weight *= 0.9;
            } else {
                node.weight *= 1.05;
            }
        }
    }

    pub fn mutate_structure(dag: &mut DAG) {
        // controlled mutation only (NOT random)
        let keys: Vec<String> = dag.nodes.keys().cloned().collect();

        for k in keys {
            if let Some(node) = dag.nodes.get_mut(&k) {
                if node.stability_score < 0.2 {
                    node.dependencies.clear(); // promote to execution-ready
                }
            }
        }
    }
}
