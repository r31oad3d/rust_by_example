use actix::prelude::*;

struct Fibonacci(pub u32);

impl Message for Fibonacci {
    type Result = Result<u64, ()>;
}

struct SyncActor;

impl Actor for SyncActor {
    type Context = SyncContext<Self>;
}

impl Handler<Fibonacci> for SyncActor {
    type Result = Result<u64, ()>;

    fn handle(
        &mut self,
        msg: Fibonacci,
        _: &mut Self::Context,
    ) -> Self::Result {
        match msg.0 {
            0 => Err(()),
            1 => Ok(1),
            _ => {
                let (mut i, mut sum, mut last, mut curr) = (0, 0, 0, 1);
                while i < msg.0 - 1 {
                    sum = last + curr;
                    last = curr;
                    curr = sum;
                    i += 1;
                }
                Ok(sum)
            }
        }
    }
}

#[actix_rt::main]
async fn main() {
    let addr = SyncArbiter::start(3, || SyncActor);
    for n in 5..10 {
        println!("{:?}", addr.send(Fibonacci(n)).await.unwrap());
    }
    System::current().stop();
}
