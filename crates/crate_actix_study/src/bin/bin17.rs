use actix::prelude::*;
use futures::{Future, FutureExt, StreamExt, TryFutureExt, TryStreamExt};

struct SumActor {}

impl Actor for SumActor {
    type Context = Context<Self>;
}

struct Value(usize, usize);

impl Message for Value {
    type Result = usize;
}

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

struct Display(usize);

impl Message for Display {
    type Result = ();
}

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: Display,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

fn main() {
    let system = System::new("single-arbiter-example");

    let sum_addr = SumActor {}.start();
    let dis_addr = DisplayActor {}.start();

    let execution = sum_addr
        .send(Value(6, 7))
        .map_err(|e| {
            eprintln!("Encountered maibox error: {:?}", e);
        })
        .and_then(move |res| {
            dis_addr.send(Display(res)).map(move |_| ()).map_err(|_| ())
        })
        .map(move |_| {
            System::current().stop();
        });
    Arbiter::spawn(execution);
    system.run();
}
