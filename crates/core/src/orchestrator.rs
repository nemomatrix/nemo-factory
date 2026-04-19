use crate::dag::TaskDAG;
use crate::state::SystemState;
use crate::execution_bus::bus::ExecutionBus;
use crate::execution_bus::context::ExecutionContext;

pub struct Orchestrator {
    pub state: SystemState,
    pub dag: TaskDAG,
    pub bus: ExecutionBus,
    pub ctx: ExecutionContext,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            state: SystemState::Init,
            dag: TaskDAG::new(),
            bus: ExecutionBus::new(),
            ctx: ExecutionContext::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.state {
                SystemState::Init => {
                    // Logic for initializing workspace
                    self.state = SystemState::SpecLocked;
                }
                SystemState::SpecLocked => {
                    // Logic for freezing specifications
                    self.state = SystemState::ArchLocked;
                }
                SystemState::ArchLocked => {
                    // Logic for verifying architecture deterministic rules
                    self.state = SystemState::DagReady;
                }
                SystemState::DagReady => {
                    // Logic for preparing the task graph
                    self.state = SystemState::Executing;
                }
                SystemState::Executing => {
                    let ready_tasks = self.dag.get_ready_tasks();
                    
                    if ready_tasks.is_empty() {
                        self.state = SystemState::Verifying;
                    } else {
                        for task in ready_tasks {
                            // Using the Execution Bus to dispatch the task
                            self.bus.send(&mut self.ctx, &format!("EXECUTE_TASK: {}", task.name));
                        }
                        
                        // Check context after bus execution
                        if self.ctx.phase == "FAILED" {
                            self.state = SystemState::Repairing;
                        }
                    }
                }
                SystemState::Verifying => {
                    self.bus.send(&mut self.ctx, "RUN_FINAL_VERIFICATION");
                    
                    if self.ctx.phase == "COMPLETED" {
                        self.state = SystemState::Completed;
                    } else {
                        self.state = SystemState::Repairing;
                    }
                }
                SystemState::Repairing => {
                    self.bus.send(&mut self.ctx, "INITIATE_SELF_HEAL_LOOP");
                    self.state = SystemState::Executing;
                }
                SystemState::Completed => {
                    println!("NEMO_FACTORY: All systems stable and operations completed.");
                    break;
                }
                SystemState::Failed => {
                    eprintln!("NEMO_FACTORY: Critical failure detected. Shutting down.");
                    break;
                }
            }
        }
    }
}
