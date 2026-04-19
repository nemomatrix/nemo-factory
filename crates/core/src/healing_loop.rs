// NEMO - The Main Self-Healing Loop
use nemo_ci_runner::runner::CiRunner;
use nemo_ci_runner::classifier::classify_error;
use crate::patch_builder::build_patch;
use crate::limiter::LoopLimiter;

pub fn run_self_healing(workspace: &str) {
    let runner = CiRunner::new(workspace);
    let mut limiter = LoopLimiter::new(10);

    loop {
        if !limiter.can_continue() {
            panic!("NEMO_FACTORY: Safety limit reached. Self-repair aborted.");
        }

        let build = runner.run_build().expect("Failed to execute build");

        if build.status.success() {
            let test = runner.run_tests().expect("Failed to execute tests");
            if test.status.success() {
                println!("STABILITY ACHIEVED: System is healthy.");
                break;
            }
        }

        let error_output = String::from_utf8_lossy(&build.stderr);
        let parsed = classify_error(&error_output);
        let patch = build_patch(parsed);

        println!("NEMO_REPAIR: Applying Patch...\n{}", patch);
        // لاحقاً سيتم ربطها بـ fs_engine لكتابة الـ Patch فعلياً
    }
}
