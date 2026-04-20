use crate::knowledge::Pattern;

pub struct CrossScorer;

impl CrossScorer {
    pub fn score(p: &Pattern) -> f32 {
        p.success_rate
    }
}
