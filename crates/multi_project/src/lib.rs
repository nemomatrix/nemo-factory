use crate::knowledge::KnowledgeBase;
use crate::pattern::PatternExtractor;
use crate::transfer::TransferEngine;
use crate::filter::Filter;

pub struct MultiProjectController;

impl MultiProjectController {
    pub fn run(kb: &mut KnowledgeBase, code: &str) {
        let pattern = PatternExtractor::extract(code);

        if Filter::allow(&pattern) {
            kb.patterns.push(pattern);
        }

        for p in &kb.patterns {
            let improved = TransferEngine::apply(p, code);
            println!("TRANSFERRED:\n{}", improved);
        }
    }
}
