pub fn main() {
    // 4.5
    //dynamic_dispatch_error::<dyn std::error::Error>().unwrap();
    _ = dynamic_dispatch();
    type_of_i32();
    type_of_closure();
}

use std::{
    any::{Any, TypeId},
    error::Error,
    fmt::Display,
};

#[derive(Debug)]
struct ErrorA;

impl Display for ErrorA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error A")
    }
}

impl Error for ErrorA {}

#[derive(Debug)]
struct ErrorB;

impl Display for ErrorB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error B")
    }
}

impl Error for ErrorB {}

fn error_a() -> Result<(), ErrorA> {
    Err(ErrorA)
}

fn error_b() -> Result<(), ErrorB> {
    Err(ErrorB)
}

// fn dynamic_dispatch_error<T>() -> Result<(), Box<T>>
// where
//     T: std::error::Error,
// {
//     error_a()?;
//     error_b()?;
//     Ok(())
// }

fn dynamic_dispatch() -> Result<(), Box<dyn std::error::Error>> {
    error_a()?;
    error_b()?;
    Ok(())
}

fn get_type_id<T: Any>(_: &T) -> TypeId {
    TypeId::of::<T>()
}

fn type_of_i32() {
    let a = 3;
    let b = 4;
    assert_eq!(get_type_id(&a), get_type_id(&b));
}

fn type_of_closure() {
    let a = |x: i32| x * x;
    let b = |x: i32| x * x;
    assert_ne!(get_type_id(&a), get_type_id(&b));
}

trait Hoge {
    fn hoge() -> String
    where
        Self: Sized,
    {
        "hoge".to_string()
    }
}

struct HogeA {}
impl Hoge for HogeA {}
struct HogeB {
    pub name: String,
}
impl Hoge for HogeB {}

fn hoge_a() -> impl Hoge {
    HogeA {}
}

fn hoge_b() -> impl Hoge {
    HogeB {
        name: "hoge_b".to_string(),
    }
}

// HogeA, HogeBは異なる型なので、if else で返している型が異なるエラーになる
// fn dispatch(b: bool) -> impl Hoge {
//     if b {
//         hoge_a()
//     } else {
//         hoge_b()
//     }
// }
