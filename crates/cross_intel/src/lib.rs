pub mod capability;
pub mod extractor;
pub mod normalizer;
pub mod selector;
pub mod integrator;
pub mod adapter;
pub mod evaluator;

use crate::extractor::Extractor;
use crate::normalizer::Normalizer;
use crate::selector::Selector;
use crate::integrator::Integrator;

pub fn run_feedback_loop() {
    let systems = vec!["Claude", "GLM"];
    let mut all_caps = vec![];

    for s in systems {
        let mut caps = Extractor::extract(s);
        for c in &mut caps {
            Normalizer::normalize(c);
        }
        all_caps.extend(caps);
    }

    let selected = Selector::select(all_caps);
    Integrator::integrate(selected);
}
