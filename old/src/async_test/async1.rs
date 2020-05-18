////#![feature(generators, generator_trait)]]
//
//extern crate futures;
//
//use futures::{executor::ThreadPool, task::SpawnExt};
//use std::future::Future;
//use std::pin::Pin;
//use std::task::*;å
//
//pub struct AlmostReady{
//    ready: bool,
//    value: i32,
//}
//
//pub fn almost_ready(value: i32) -> AlmostReady {
//    AlmostReady{ready: false, value}
//}
//
//impl Future for AlmostReady {
//    type Output = i32;
//    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
//        if self.ready {
//            Poll::Ready(self.value + 1)
//        } else {
//            unsafe {
//                Pin::get_unchecked_mut(self).ready = true;
//            }
//            lw.wake();
//            Poll::Pending
//        }
//    }
//}
//
//pub fn test1() {
//    let mut executor = ThreadPool::new().unwrap();
//    let future = async {
//        println!("howdy");
//        let x = almost_ready(5).await;
//        println!("done: {:?}", x);
//    };
//    executor.run(future);
//}
