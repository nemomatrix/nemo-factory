pub struct Invariants;

impl Invariants {
    pub fn enforce_single_entry(called_from_bus: bool) {
        if !called_from_bus {
            panic!("INVARIANT VIOLATION: execution outside bus");
        }
    }

    pub fn enforce_phase_lock(phase_locked: bool, attempting_write: bool) {
        if phase_locked && attempting_write {
            panic!("INVARIANT VIOLATION: write in locked phase");
        }
    }

    pub fn enforce_ci_truth(ci_verified: bool) {
        if !ci_verified {
            panic!("INVARIANT VIOLATION: unverified execution");
        }
    }
}
