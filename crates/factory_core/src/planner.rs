pub struct Plan {
    pub tasks: Vec<String>,
}

pub struct Planner;

impl Planner {
    pub fn build_plan(spec: &str) -> Plan {
        let tasks = vec![
            "init_workspace".into(),
            "compile_core".into(),
            "run_tests".into(),
            "validate_ci".into(),
        ];

        Plan { tasks }
    }
}
