pub mod randomized_vec;
pub mod xor64;

pub fn main() {
    join_test();
    multiple_join_test();

    channel_multiple_sender();
    _channel_multiple_receiver();

    sort_vec_single_thread();
    sort_vec_multi_thread();
}

fn join_test() {
    let handler = std::thread::spawn(|| {
        println!("worker");
        100
    });
    println!("main");
    match handler.join() {
        Ok(n) => println!("return: {n}"),
        Err(e) => println!("{:?}", e),
    }
}

fn multiple_join_test() {
    let handler1 = std::thread::spawn(|| {
        println!("worker1");
        1
    });
    let handler2 = std::thread::spawn(|| {
        println!("worker2");
        2
    });
    let handler3 = std::thread::spawn(|| {
        println!("worker3");
        3
    });
    let handler4 = std::thread::spawn(|| {
        println!("worker4");
        4
    });
    let handler5 = std::thread::spawn(|| {
        println!("worker5");
        5
    });
    println!("main");
    match (
        handler1.join(),
        handler2.join(),
        handler3.join(),
        handler4.join(),
        handler5.join(),
    ) {
        (Ok(n1), Ok(n2), Ok(n3), Ok(n4), Ok(n5)) => {
            println!("return1: {n1}, return2: {n2}, return3: {n3}, return4: {n4}, return5: {n5}")
        }
        _ => println!("error occur!"),
    }
}

use std::{thread, time};

use rand::thread_rng;
fn channel_multiple_sender() {
    let (sender, receiver) = std::sync::mpsc::sync_channel(64);

    let i_vec = (0..30).collect::<Vec<i32>>();
    let handlers: Vec<_> = i_vec
        .into_iter()
        .map({
            let ss = sender.clone();
            move |i| {
                std::thread::spawn({
                    let s = ss.clone();
                    move || {
                        thread::sleep(time::Duration::from_millis(100));
                        s.send(i)
                    }
                })
            }
        })
        .collect();
    handlers.into_iter().for_each(|h| match h.join() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    });
    let mut result: Vec<i32> = Vec::new();
    for _ in 0..30 {
        match receiver.recv() {
            Ok(i) => result.push(i),
            Err(e) => println!("{:?}", e),
        }
    }
    println!("result: {:?}", result);
}

fn _channel_multiple_receiver() {
    // https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
    // The receiving half of Rust’s channel (or sync_channel) type. This half can only be owned by one thread.
    // と書かれているようにReceiverは複数スレッドで共有できない
    // Senderは、
    // This half can only be owned by one thread, but it can be cloned to send to other threads.
    // と書かれており、共有でできないがcloneができる
}

fn sort_vec_single_thread() {
    let mut v1 = randomized_vec::randomized_vec(1234);
    let mut v2 = randomized_vec::randomized_vec(6789);

    let start = std::time::Instant::now();

    v1.sort();
    v2.sort();

    let end = start.elapsed();
    println!(
        "single threaded: {}.{:03}sec",
        end.as_secs(),
        end.subsec_nanos()
    );
}

fn sort_vec_multi_thread() {
    let mut v1 = randomized_vec::randomized_vec(1234);
    let mut v2 = randomized_vec::randomized_vec(6789);

    let start = std::time::Instant::now();
    let handler1 = thread::spawn(|| {
        v1.sort();
        v1
    });
    let handler2 = thread::spawn(|| {
        v2.sort();
        v2
    });

    match (handler1.join(), handler2.join()) {
        (Ok(_), Ok(_)) => (),
        _ => println!("error!: sthread 1 or 2 "),
    };
    let end = start.elapsed();
    println!(
        "multi threaded: {}.{:03}sec",
        end.as_secs(),
        end.subsec_nanos()
    );
}
