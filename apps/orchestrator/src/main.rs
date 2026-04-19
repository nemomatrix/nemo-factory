use crate::planner::Planner;
use crate::factory::Factory;

pub fn run_factory(spec: &str) {
    let planner = Planner;
    let mut factory = Factory::new();

    loop {
        let plan = planner.build_plan(spec);

        factory.run_cycle(plan);

        // stop condition لاحقاً CI success
        break;
    }
}
