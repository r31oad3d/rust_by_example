use actix::*;

use std::env;
use std::time::SystemTime;

struct Payload(usize);

impl Message for Payload {
    type Result = ();
}

struct Node {
    id: usize,
    limit: usize,
    next: Recipient<Payload>,
}

impl Actor for Node {
    type Context = Context<Self>;
}

impl Handler<Payload> for Node {
    type Result = ();

    fn handle(&mut self, msg: Payload, _: &mut Context<Self>) {
        if msg.0 >= self.limit {
            println!(
                "Actor {} reached limit of {} (paylod was {})",
                self.id, self.limit, msg.0
            );
            System::current().stop();
            return;
        }

        if msg.0 % 498989 == 1 {
            println!(
                "Actor {} received message {} of {} ({:.2}%)",
                self.id,
                msg.0,
                self.limit,
                100.0 * msg.0 as f32 / self.limit as f32
            );
        }
        self.next
            .do_send(Payload(msg.0 + 1))
            .expect("Unable tp send payload");
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage; bin18 <num_nodes> <num-times_message_around_ring>");
    ::std::process::exit(1);
}

fn main() -> std::io::Result<()> {
    let system = System::new("ring");
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        print_usage_and_exit();
    }
    let n_node = if let Ok(arg_num_nodes) = args[1].parse::<usize>() {
        if arg_num_nodes <= 1 {
            eprintln!("Number of nodes must be > 1");
            ::std::process::exit(1);
        }
        arg_num_nodes
    } else {
        print_usage_and_exit();
    };

    let n_times = if let Ok(arg_times) = args[2].parse::<usize>() {
        arg_times
    } else {
        print_usage_and_exit()
    };

    let setup = SystemTime::now();

    println!("Setting ip {} nodes", n_node);
    let limit = n_node * n_times;
    let node = Node::create(move |ctx| {
        let first_addr = ctx.address();
        let mut prev_addr = Node {
            id: 1,
            limit,
            next: first_addr.recipient(),
        }
        .start();

        for id in 2..n_node {
            prev_addr = Node {
                id,
                limit,
                next: prev_addr.recipient(),
            }
            .start();
        }

        Node {
            id: n_node,
            limit,
            next: prev_addr.recipient(),
        }
    });

    match setup.elapsed() {
        Ok(elapsed) => println!(),
        Err(e) => println!("An error occurred: {:?} ", e),
    }

    println!(
        "Sending start message and waiting for termination after {} message",
        limit
    );

    let now = SystemTime::now();

    let _req = node.send(Payload(1));

    match system.run() {
        Ok(_) => println!("Complete"),
        Err(e) => println!("An error occurred: {:?}", e),
    }

    match now.elapsed() {
        Ok(elapsed) => println!(
            "Time taken: {}.{:06} seconds ({} msg/second)",
            elapsed.as_secs(),
            elapsed.subsec_micros(),
            (n_node * n_times * 1000000) as u128 / elapsed.as_micros()
        ),
        Err(e) => println!("An error occurred: {:?}", e),
    }

    Ok(())
}
