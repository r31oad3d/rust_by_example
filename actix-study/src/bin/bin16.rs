extern crate actix;
extern crate futures;
use actix::prelude::*;
use futures::Future;
struct MyActor {
    count: usize,
}
impl Actor for MyActor {
    type Context = Context<Self>;
}

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        if self.count > 5 {
            println!("Shutting down ping receiver.");
            ctx.stop()
        }

        self.count
    }
}

fn main() {
    let system = System::new("test");

    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let addr_2 = addr.clone();
    let res = addr.send(Ping(6));

    Arbiter::spawn(
        res.map(move |res| {
            // Now, the ping actor should have stopped, so a second message will fail
            // With a SendError::Closed
            assert!(addr_2.try_send(Ping(6)).is_err());

            // Shutdown gracefully now.
            System::current().stop();
        })
        .map_err(|_| ()),
    );

    system.run();
}
