pub mod graph;
pub mod node;
pub mod compiler;
pub mod evaluator;
pub mod evolution;
pub mod scorer;
pub mod lock;

pub use graph::DAG;
pub use compiler::Compiler;
pub use evolution::Evolver;
pub use lock::Lock;

use crate::graph::DAG as InternalDAG;
use crate::compiler::Compiler as InternalCompiler;
use crate::evolution::Evolver as InternalEvolver;
use crate::lock::Lock as InternalLock;

pub fn run_evolving_system(dag: &mut InternalDAG) {
    loop {
        // Enforce safety constraints and DAG size
        InternalLock::enforce(dag.nodes.len()).unwrap();

        // Compile DAG into an execution plan
        let plan = InternalCompiler::compile(dag);

        for step in plan {
            println!("EXECUTING_STEP: {}", step);
        }

        // Placeholder for CI result from ci_runner
        let ci_result = true; 

        // Evolve nodes and mutate DAG structure based on result
        InternalEvolver::evolve(dag, ci_result);
        InternalEvolver::mutate_structure(dag);

        if ci_result {
            println!("SYSTEM_STABLE_STATE_ACHIEVED");
            break;
        }
    }
}
