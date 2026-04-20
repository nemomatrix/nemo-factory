use crate::proposal::Proposal;

pub struct Commit;

impl Commit {
    pub fn apply(p: &Proposal) {
        println!("COMMIT CHANGE: {}", p.target);
        // ربط مع DAG mutation layer
    }
}
