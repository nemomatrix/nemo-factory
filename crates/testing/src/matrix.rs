use crate::case::TestCase;

pub fn generate() -> Vec<TestCase> {
    let phases = ["core", "exec", "recovery", "integration"];
    let failures = ["none", "compile", "timeout", "corrupt", "partial"];
    let loads = [1, 10, 100];
    let sizes = [5, 50, 200];

    let mut cases = vec![];

    for p in phases {
        for f in failures {
            for l in loads {
                for s in sizes {
                    cases.push(TestCase {
                        phase: p.into(),
                        failure: f.into(),
                        load: l,
                        dag_size: s,
                        seed: 42,
                    });
                }
            }
        }
    }

    cases
}
