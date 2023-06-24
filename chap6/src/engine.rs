mod codegen;
mod evaluator;
mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Caret,
    Doller,
    Match,
    Jump(usize),
    Split(usize, usize, (i32, i32), i32),
    Descrement(usize),
    AnyNumber,
    NotNumber,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Caret => write!(f, "caret"),
            Instruction::Doller => write!(f, "doller"),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2, count, is_register_idx_increment) => {
                write!(
                    f,
                    "split {:>04}, {:>04}, {:?}, {}",
                    addr1, addr2, count, is_register_idx_increment
                )
            }
            Instruction::Descrement(idx) => write!(f, "decrement {}", idx),
            Instruction::AnyNumber => write!(f, "any number"),
            Instruction::NotNumber => write!(f, "not number"),
        }
    }
}

pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST: {:?}", ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }

    Ok(())
}

pub fn do_matching(expr: &str, line: &str, index: usize, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, index, is_depth)?)
}
