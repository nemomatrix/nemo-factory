use crate::context::ExecutionContext;
use crate::dispatcher::Dispatcher;
use crate::guard::Guard;

pub struct ExecutionBus {
    dispatcher: Dispatcher,
}

impl ExecutionBus {
    pub fn new() -> Self {
        Self {
            dispatcher: Dispatcher,
        }
    }

    pub fn send(&self, ctx: &mut ExecutionContext, input: &str) {
        if let Err(e) = Guard::validate(ctx) {
            panic!("BUS BLOCKED: {}", e);
        }

        self.dispatcher.execute(ctx, input);
    }
}
