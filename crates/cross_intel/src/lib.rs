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
