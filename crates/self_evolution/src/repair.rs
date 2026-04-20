use crate::weakness::Weakness;

pub struct Repair;

impl Repair {
    pub fn propose(w: &Weakness) -> Option<String> {
        if w.severity < 0.1 {
            return None;
        }

        match w.kind.as_str() {
            "TIMEOUT" => Some("increase timeout limit".into()),
            "COMPILE" => Some("fix build dependency".into()),
            _ => None,
        }
    }
}
