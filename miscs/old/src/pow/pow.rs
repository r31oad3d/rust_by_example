//extern crate itertools;
//extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use itertools::Itertools;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

const BASE: usize = 42_usize;
const THREADS: usize = 8_usize;
static DIFFICULTY: &'static str = "0";

struct Solution(usize, String);

fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string());
    let hash = hasher.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

fn find(
    start_at: usize,
    sender: mpsc::Sender<Solution>,
    is_solution_found: Arc<AtomicBool>,
) {
    for number in (start_at..).step_by(THREADS) {
        if is_solution_found.load(Ordering::Relaxed) {
            return;
        }
        if let Some(solution) = verify(number) {
            is_solution_found.store(true, Ordering::Relaxed);
            sender.send(solution).unwrap();
            return;
        }
    }
}
pub fn do_pow() {
    println!(
        "PoW: find a number, SHA256(the number * {}) == \"{}......\"",
        BASE, DIFFICULTY
    );
    println!("Started {} threads", THREADS);
    println!("Please wait...  ");
    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();
    for i in 00..THREADS {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            find(i, sender_n, is_solution_found);
        });
    }

    match receiver.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution:");
            println!("The number is: {}, and the hash result is {}", i, hash);
        }
        Err(_) => {
            panic!("Worker threads disconnected!");
        }
    }
}
