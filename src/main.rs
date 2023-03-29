pub fn main() {
    join_test();
    multiple_join_test();
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
