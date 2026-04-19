use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub dependencies: Vec<Uuid>,
    pub completed: bool,
}

impl Task {
    pub fn new(name: &str, dependencies: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            dependencies,
            completed: false,
        }
    }
}
