use crate::knowledge::Pattern;

pub struct Filter;

impl Filter {
    pub fn allow(p: &Pattern) -> bool {
        p.success_rate > 0.6
    }
}
