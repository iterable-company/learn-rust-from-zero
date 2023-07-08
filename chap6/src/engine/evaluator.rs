use super::Instruction;
use crate::helper::safe_add;
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    index: usize,
    mut pc: usize,
    mut sp: usize,
    mut register: Vec<(i32, Option<i32>)>,
    cache: &mut HashSet<(usize, usize)>,
    matched_str: &mut Vec<Vec<char>>,
    mut capcher_expected: bool,
) -> Result<bool, EvalError> {
    println!(
        "eval_depth:: inst: {:?}, line: {:?}, index: {}, pc: {}, sp: {}",
        inst, line, index, pc, sp
    );
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };
        println!(
            "next: {:?}, pc: {}, sp: {}, register: {:?}",
            next, pc, sp, register
        );

        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c || *c == '.' {
                        if capcher_expected {
                            matched_str.last_mut().unwrap().push(*sp_c);
                        }
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::UnmatchChars(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if !c.contains(sp_c) {
                        if capcher_expected {
                            matched_str.last_mut().unwrap().push(*sp_c);
                        }
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                }
            }
            Instruction::AnyNumber => {
                if let Some(sp_c) = line.get(sp) {
                    if "0123456789".chars().collect::<Vec<_>>().contains(sp_c) {
                        if capcher_expected {
                            matched_str.last_mut().unwrap().push(*sp_c);
                        }
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::NotNumber => {
                if let Some(sp_c) = line.get(sp) {
                    if !"0123456789".chars().collect::<Vec<_>>().contains(sp_c) {
                        if capcher_expected {
                            matched_str.last_mut().unwrap().push(*sp_c);
                        }
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Caret => {
                if sp != 0 || index != 0 {
                    return Ok(false);
                }
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
            }
            Instruction::Doller => {
                if sp != line.len() {
                    return Ok(false);
                }
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
            }
            Instruction::Match => {
                return Ok(register.iter().all(|counter| {
                    counter.0 <= 0 && (counter.1.is_none() || counter.1.unwrap() >= 0)
                }))
            }
            Instruction::Jump(addr) => {
                if cache.contains(&(*addr, sp)) {
                    return Ok(false);
                }
                cache.insert((*addr, sp));
                pc = *addr
            }
            Instruction::Split(addr1, addr2, count, register_idx) => {
                if *register_idx >= 0 {
                    if let Some(_) = register.get_mut(*register_idx as usize) {
                    } else {
                        register.push(*count);
                    }
                }
                if eval_depth(inst, line, index, *addr1, sp, register.clone(), cache, matched_str, capcher_expected)?
                    || eval_depth(inst, line, index, *addr2, sp, register, cache, matched_str, capcher_expected)?
                {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Instruction::Descrement(idx) => {
                if let Some(c) = register[*idx].1 {
                    if c == 0 {
                        return Ok(false);
                    }
                    register[*idx].1 = Some(c - 1);
                }
                register[*idx].0 = register[*idx].0 - 1;
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
            }
            Instruction::CapcherBegin(register_idx) => {
                if *register_idx >= 0 {
                    if let Some(_) = matched_str.get_mut(*register_idx as usize) {
                    } else {
                        matched_str.push(vec![]);
                    }
                    capcher_expected = true;
                }
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
            }
            Instruction::CapcherEnd(_) => {
                capcher_expected = false;
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
            }
        }
    }
}

fn eval_width(inst: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    let mut ctx = VecDeque::new();
    let mut pc = 0;
    let mut sp = 0;

    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };
        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c || *c == '.' {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        if ctx.is_empty() {
                            return Ok(false);
                        } else {
                            pop_ctx(&mut pc, &mut sp, &mut ctx)?;
                        }
                    }
                } else {
                    if ctx.is_empty() {
                        return Ok(false);
                    } else {
                        pop_ctx(&mut pc, &mut sp, &mut ctx)?;
                    }
                }
            }
            Instruction::UnmatchChars(_) => {
                todo!()
            }
            Instruction::AnyNumber => {
                todo!()
            }
            Instruction::NotNumber => {
                todo!()
            }
            Instruction::Caret => {
                if sp != 0 {
                    return Ok(false);
                }
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
            }
            Instruction::Doller => {
                if sp != line.len() {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Split(addr1, addr2, _count, _is_register_idx_increment) => {
                pc = *addr1;
                ctx.push_back((*addr2, sp));
                continue;
            }
            Instruction::Descrement(_register_idx) => todo!(),
            Instruction::CapcherBegin(_) => todo!(),
            Instruction::CapcherEnd(_) => todo!(),
        }

        if !ctx.is_empty() {
            ctx.push_back((pc, sp));
            pop_ctx(&mut pc, &mut sp, &mut ctx)?;
        }
    }
}

fn pop_ctx(
    pc: &mut usize,
    sp: &mut usize,
    ctx: &mut VecDeque<(usize, usize)>,
) -> Result<(), EvalError> {
    if let Some((p, s)) = ctx.pop_back() {
        *pc = p;
        *sp = s;
        Ok(())
    } else {
        Err(EvalError::InvalidContext)
    }
}

pub fn eval(
    inst: &[Instruction],
    line: &[char],
    index: usize,
    is_depth: bool,
) -> Result<(bool, Vec<String>), EvalError> {
    if is_depth {
        let register = Vec::<(i32, Option<i32>)>::new();
        let mut cache = HashSet::<(usize, usize)>::new();
        let mut matched_str = Vec::<Vec<char>>::new();
        println!("inst: {:?}", inst);
        let is_match = eval_depth(inst, line, index, 0, 0, register, &mut cache, &mut matched_str, false)?;
        Ok((is_match, matched_str.into_iter().map(|vec_char| vec_char.into_iter().collect()).collect()))
        //Ok((false, vec![]))
    } else {
        eval_width(inst, line);
        todo!()
    }
}
