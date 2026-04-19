use crate::dag::TaskDAG;
use crate::state::SystemState;

pub struct Orchestrator {
    pub state: SystemState,
    pub dag: TaskDAG,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            state: SystemState::Init,
            dag: TaskDAG::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.state {
                SystemState::Init => {
                    self.state = SystemState::SpecLocked;
                }
                SystemState::SpecLocked => {
                    self.state = SystemState::ArchLocked;
                }
                SystemState::ArchLocked => {
                    self.state = SystemState::DagReady;
                }
                SystemState::DagReady => {
                    self.state = SystemState::Executing;
                }
                SystemState::Executing => {
                    let ready = self.dag.get_ready_tasks();
                    if ready.is_empty() {
                        self.state = SystemState::Verifying;
                    } else {
                        for task in ready {
                            println!("Executing task: {}", task.name);
                        }
                    }
                }
                SystemState::Verifying => {
                    self.state = SystemState::Completed;
                }
                SystemState::Repairing => {
                    self.state = SystemState::Executing;
                }
                SystemState::Completed => break,
                SystemState::Failed => break,
            }
        }
    }
}
