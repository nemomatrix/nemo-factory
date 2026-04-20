pub struct Pattern {
    pub signature: String,
    pub success_rate: f32,
}

pub struct KnowledgeBase {
    pub patterns: Vec<Pattern>,
}
