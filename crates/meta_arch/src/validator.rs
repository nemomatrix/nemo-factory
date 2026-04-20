use crate::proposal::Proposal;

pub struct Validator;

impl Validator {
    pub fn validate(p: &Proposal) -> Result<(), String> {
        if p.score < 0.2 {
            return Err("LOW CONFIDENCE CHANGE BLOCKED".into());
        }

        Ok(())
    }
}
