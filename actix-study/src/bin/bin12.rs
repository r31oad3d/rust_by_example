use actix::prelude::*;
use futures::{FutureExt, TryFutureExt};

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
        self.count
    }
}

fn main() -> std::io::Result<()> {
    let system = System::new("test");
    let addr = MyActor { count: 10 }.start();
    let res = addr.send(Ping(10));
    Arbiter::spawn(
        res.map(|res| {
            println!("RESULT: {}", res.unwrap() == 20);
        })
        .map_err(|_| ()),
    );

    system.run()
}
