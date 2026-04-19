use crate::task::Task;
use std::collections::HashMap;

pub struct TaskDAG {
    pub tasks: HashMap<String, Task>,
}

impl TaskDAG {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.name.clone(), task);
    }

    pub fn get_ready_tasks(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| {
                !t.completed &&
                t.dependencies.iter().all(|dep_id| {
                    self.tasks.values().any(|t2| t2.id == *dep_id && t2.completed)
                })
            })
            .collect()
    }
}
