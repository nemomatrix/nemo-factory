use crate::graph::DAG;

pub struct Compiler;

impl Compiler {
    pub fn compile(dag: &DAG) -> Vec<String> {
        let mut plan = vec![];

        for node in dag.get_ready_nodes() {
            plan.push(format!("EXEC_NODE: {}", node.name));
        }

        plan
    }
}
