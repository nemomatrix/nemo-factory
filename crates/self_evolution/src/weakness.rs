pub struct Weakness {
    pub kind: String,
    pub severity: f32,
}

pub fn detect_weakness(clusters: std::collections::HashMap<String, usize>, total: usize)
    -> Vec<Weakness>
{
    clusters.into_iter().map(|(k, v)| {
        Weakness {
            kind: k,
            severity: score(v, total),
        }
    }).collect()
}
