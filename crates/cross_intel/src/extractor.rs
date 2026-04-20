use crate::capability::Capability;

pub struct Extractor;

impl Extractor {
    pub fn extract(system_name: &str) -> Vec<Capability> {
        match system_name {
            "Claude" => vec![
                Capability {
                    name: "LongContextReasoning".into(),
                    domain: "reasoning".into(),
                    strength: 0.9,
                    cost: 0.7,
                    latency: 300,
                }
            ],
            "GLM" => vec![
                Capability {
                    name: "MultilingualGeneration".into(),
                    domain: "generation".into(),
                    strength: 0.85,
                    cost: 0.5,
                    latency: 200,
                }
            ],
            _ => vec![],
        }
    }
}
