use crate::invariants::Invariants;
use crate::audit::Audit;

pub struct Supervisor;

impl Supervisor {
    pub fn monitor() {
        Audit::log("SYSTEM CHECK START");

        Invariants::enforce_ci_truth(true);

        Audit::log("SYSTEM STABLE");
    }
}
