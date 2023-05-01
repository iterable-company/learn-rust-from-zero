pub fn main() {
    // 4.5
    //dynamic_dispatch_error::<dyn std::error::Error>().unwrap();
    dynamic_dispatch().unwrap();
}

use std::{error::Error, fmt::Display};

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
