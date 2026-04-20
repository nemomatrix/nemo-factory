
pub struct Gate;

impl Gate {
    pub fn allow(severity: f32) -> bool {
        severity > 0.2 && severity < 0.8
    }
}
