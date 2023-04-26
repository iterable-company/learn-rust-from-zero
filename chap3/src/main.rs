pub fn main() {
    // 3.4
    borrow();
    destructive_assignment();

    // 3.5
    read_write_lock();
}
#[derive(Debug)]
struct XY {
    x: Vec<i32>,
    y: Vec<i32>,
}
fn borrow() {
    let mut xy = XY {
        x: vec![1],
        y: Vec::new(),
    };
    for elm in xy.x.iter() {
        xy.y.push(*elm + *elm);
    }
    println!("{:?}", xy);
}

fn destructive_assignment() {
    let mut xy = XY {
        x: vec![1],
        y: Vec::new(),
    };
    let XY { x, y } = &mut xy;
    for elm in x.iter() {
        y.push(*elm + *elm);
    }
    println!("{:?}", xy);
}

fn read_write_lock() {
    use itertools::Itertools;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
        thread::sleep,
        time::Duration,
    };

    let gallery = HashMap::from([
        ("葛飾北斎", "富嶽三十六景 神奈川沖浪裏"),
        ("ミュシャ", "黄道十二宮"),
    ]);
    let gallery = Arc::new(RwLock::new(gallery));

    let mut hdls = vec![];
    for n in 0..3 {
        let gallery = gallery.clone();
        let hdl = std::thread::spawn(move || {
            for m in 0..8 {
                let guard = gallery.read().unwrap();
                if n % 2 == 0 {
                    println!(
                        "{}",
                        guard
                            .iter()
                            .map(|(key, value)| format!("{n},{m}, {key}:{value}"))
                            .join(", ")
                    );
                }
                sleep(Duration::from_secs(1));
            }
        });
        hdls.push(hdl);
    }
    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
            } else {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
                guard.insert("ミュシャ", "黄道十二宮");
            }
            sleep(Duration::from_secs(2));
        }
    });
    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();
}
