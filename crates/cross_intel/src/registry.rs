pub struct ExternalSystem {
    pub name: String,
    pub capabilities: Vec<Capability>,
}

pub struct Registry {
    pub systems: Vec<ExternalSystem>,
}
