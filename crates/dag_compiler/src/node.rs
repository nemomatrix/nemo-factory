#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub dependencies: Vec<String>,
    pub weight: f32,
    pub stability_score: f32,
}
