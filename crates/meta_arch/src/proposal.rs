#[derive(Clone)]
pub enum ChangeType {
    SplitNode,
    MergeNodes,
    ReorderDependencies,
    RemoveNode,
}

#[derive(Clone)]
pub struct Proposal {
    pub change: ChangeType,
    pub target: String,
    pub score: f32,
}
