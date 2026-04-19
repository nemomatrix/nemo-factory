use crate::context::ExecutionContext;
use crate::router::{route, Route};

pub struct Dispatcher;

impl Dispatcher {
    pub fn execute(&self, ctx: &mut ExecutionContext, input: &str) {
        match route(input) {
            Route::Build => {
                self.handle_build(ctx);
            }

            Route::Test => {
                self.handle_test(ctx);
            }

            Route::Patch => {
                self.handle_patch(ctx);
            }

            Route::Heal => {
                self.handle_heal(ctx);
            }

            Route::Idle => {}
        }
    }

    fn handle_build(&self, _ctx: &mut ExecutionContext) {
        println!("EXEC: BUILD PIPELINE");
    }

    fn handle_test(&self, _ctx: &mut ExecutionContext) {
        println!("EXEC: TEST PIPELINE");
    }

    fn handle_patch(&self, _ctx: &mut ExecutionContext) {
        println!("EXEC: PATCH PIPELINE");
    }

    fn handle_heal(&self, _ctx: &mut ExecutionContext) {
        println!("EXEC: HEALING PIPELINE");
    }
}
