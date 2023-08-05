use super::{parser::AST, Instruction};
use crate::helper::safe_add;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum CodeGenError {
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
    FailCounter,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for CodeGenError {}

#[derive(Default, Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

impl Generator {
    fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        let mut register_idx = 0;
        let mut register_match_str_idx = 0;
        self.gen_expr(ast, &mut register_idx, &mut register_match_str_idx)?;
        self.inc_pc()?;
        self.insts.push(Instruction::Match);
        Ok(())
    }

    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverFlow)
    }

    fn gen_expr(&mut self, ast: &AST, register_idx: &mut i32, register_match_str_idx: &mut i32) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::UnmatchChars(c) => self.gen_unmacth_chars(c.to_vec())?,
            AST::Or(e1, e2) => self.gen_or(e1, e2, register_idx, register_match_str_idx)?,
            AST::Plus(e) => self.gen_plus(e, register_idx, register_match_str_idx)?,
            AST::Star(e) => self.gen_star(e, register_idx, register_match_str_idx)?,
            AST::Question(e) => self.gen_question(e, register_idx, register_match_str_idx)?,
            AST::Seq(v) => self.gen_seq(v, register_idx, register_match_str_idx)?,
            AST::Caret => self.gen_caret()?,
            AST::Doller => self.gen_doller()?,
            AST::AnyNumber => self.get_number()?,
            AST::NotNumber => self.get_not_number()?,
            AST::Counter(e, count) => self.gen_counter(e, *count, register_idx, register_match_str_idx)?,
            AST::Chapcher(e) => self.gen_capcher(e, register_idx, register_match_str_idx)?,
        }
        Ok(())
    }

    fn gen_seq(&mut self, exprs: &[AST], register_idx: &mut i32, register_match_str_idx: &mut i32) -> Result<(), CodeGenError> {
        for e in exprs {
            self.gen_expr(e, register_idx, register_match_str_idx)?;
        }
        Ok(())
    }

    fn get_number(&mut self) -> Result<(), CodeGenError> {
        let inst = Instruction::AnyNumber;
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn get_not_number(&mut self) -> Result<(), CodeGenError> {
        let inst = Instruction::NotNumber;
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_unmacth_chars(&mut self, c: Vec<char>) -> Result<(), CodeGenError> {
        let inst = Instruction::UnmatchChars(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_caret(&mut self) -> Result<(), CodeGenError> {
        let inst = Instruction::Caret;
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_doller(&mut self) -> Result<(), CodeGenError> {
        let inst = Instruction::Doller;
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_or(&mut self, e1: &AST, e2: &AST, register_idx: &mut i32, register_match_str_idx: &mut i32) -> Result<(), CodeGenError> {
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0, (-1, None), -1);
        self.insts.push(split);

        self.gen_expr(e1, register_idx, register_match_str_idx)?;

        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0));

        self.inc_pc()?;
        if let Some(Instruction::Split(_, l2, (-1, None), -1)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        self.gen_expr(e2, register_idx, register_match_str_idx)?;

        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jmp_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_question(&mut self, e: &AST, register_idx: &mut i32, register_match_str_idx: &mut i32) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0, (-1, None), -1); // self.pcがL1。L2を仮に0と設定
        self.insts.push(split);

        // L1: eのコード
        self.gen_expr(e, register_idx, register_match_str_idx)?;

        // L2の値を設定
        if let Some(Instruction::Split(_, l2, (-1, None), -1)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailQuestion)
        }
    }

    fn gen_plus(&mut self, e: &AST, register_idx: &mut i32, register_match_str_idx: &mut i32) -> Result<(), CodeGenError> {
        // L1: eのコード
        let l1 = self.pc;
        self.gen_expr(e, register_idx, register_match_str_idx)?;

        // split L1, L2
        self.inc_pc()?;
        let split = Instruction::Split(l1, self.pc, (-1, None), -1); // self.pcがL2
        self.insts.push(split);

        Ok(())
    }

    fn gen_star(&mut self, e: &AST, register_idx: &mut i32, register_match_str_idx: &mut i32) -> Result<(), CodeGenError> {
        // L1: split L2, L3
        let l1 = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0, (-1, None), -1); // self.pcがL2。L3を仮に0と設定
        self.insts.push(split);

        // L2: eのコード
        self.gen_expr(e, register_idx, register_match_str_idx)?;

        // jump L1
        self.inc_pc()?;
        self.insts.push(Instruction::Jump(l1));

        // L3の値を設定
        if let Some(Instruction::Split(_, l3, _, _)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailStar)
        }
    }

    fn gen_counter(
        &mut self,
        e: &AST,
        count: (usize, Option<usize>),
        register_idx: &mut i32,
        register_match_str_idx: &mut i32,
    ) -> Result<(), CodeGenError> {
        // L1: split L2, L3
        println!("e: {:?}, count: {:?}, register_idx: {}", e, count, register_idx);
        let l1 = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(
            self.pc,
            0,
            (count.0 as i32, count.1.map(|c| c as i32)),
            *register_idx,
        ); // self.pcがL2。L3を仮に0と設定
        self.insts.push(split);
        let decrement = Instruction::Descrement(*register_idx as usize);
        *register_idx += 1;
        self.insts.push(decrement);
        self.inc_pc()?;

        // L2: eのコード
        self.gen_expr(e, register_idx, register_match_str_idx)?;

        // jump L1
        self.inc_pc()?;
        self.insts.push(Instruction::Jump(l1));

        // L3の値を設定
        if let Some(Instruction::Split(_, l3, _, _)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailCounter)
        }
    }

    fn gen_capcher(
        &mut self,
        e: &AST,
        register_idx: &mut i32,
        register_match_str_idx: &mut i32,
    ) -> Result<(), CodeGenError> {
        let idx = *register_match_str_idx;
        let inst = Instruction::CapcherBegin(idx);
        *register_match_str_idx += 1;
        self.insts.push(inst);
        self.inc_pc()?;

        self.gen_expr(e, register_idx, register_match_str_idx)?;
        self.inc_pc()?;
        let inst = Instruction::CapcherEnd(idx);
        self.insts.push(inst);

        Ok(())
    }
}

pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}

#[cfg(test)]
mod tests {
    use super::{get_code, AST};

    use crate::engine::Instruction::*;
    #[test]
    fn test_star() {
        let instructions = get_code(&AST::Seq(vec![
            AST::Char('a'),
            AST::Star(Box::new(AST::Or(
                Box::new(AST::Seq(vec![AST::Char('b'), AST::Char('c')])),
                Box::new(AST::Seq(vec![AST::Char('d'), AST::Char('e')])),
            ))),
            AST::Char('f'),
        ]))
        .unwrap(); //"a(bc|de)*f"
        assert_eq!(instructions, vec![
            Char('a'),
            Split(2, 9, (-1, None), -1),
            Split(3, 6, (-1, None), -1),
            Char('b'),
            Char('c'),
            Jump(8),
            Char('d'),
            Char('e'),
            Jump(1),
            Char('f'),
            Match,
        ]);
    }

    #[test]
    fn test_question() {
        let instructions = get_code(&AST::Seq(vec![
            AST::Char('a'),
            AST::Question(Box::new(AST::Or(
                Box::new(AST::Seq(vec![AST::Char('b'), AST::Char('c')])),
                Box::new(AST::Seq(vec![AST::Char('d'), AST::Char('e')])),
            ))),
            AST::Char('f'),
        ]))
        .unwrap(); //"a(bc|de)?f"
        assert_eq!(instructions, vec![
            Char('a'),
            Split(2, 8, (-1, None), -1),
            Split(3, 6, (-1, None), -1),
            Char('b'),
            Char('c'),
            Jump(8),
            Char('d'),
            Char('e'),
            Char('f'),
            Match,
        ]);
    }

    #[test]
    fn test_plus() {
        let instructions = get_code(&AST::Seq(vec![
            AST::Char('a'),
            AST::Plus(Box::new(AST::Or(
                Box::new(AST::Seq(vec![AST::Char('b'), AST::Char('c')])),
                Box::new(AST::Seq(vec![AST::Char('d'), AST::Char('e')])),
            ))),
            AST::Char('f'),
        ]))
        .unwrap(); //"a(bc|de)+f"
        assert_eq!(instructions, vec![
            Char('a'),
            Split(2, 5, (-1, None), -1),
            Char('b'),
            Char('c'),
            Jump(7),
            Char('d'),
            Char('e'),
            Split(1, 8, (-1, None), -1),
            Char('f'),
            Match,
        ]);
    }

    #[test]
    fn test_unmatch() {
        let instructions = get_code(&AST::Seq(vec![
            AST::Char('a'),
            AST::Char('b'),
            AST::Counter(
                Box::new(AST::UnmatchChars(vec!['c','d'])), (2, Some(2))
            )
        ])).unwrap(); //"ab[^cd]{2}"
        assert_eq!(instructions, vec![
            Char('a'),
            Char('b'),
            Split(3, 6, (2, Some(2)), 0),
            Descrement(0),
            UnmatchChars(vec!['c','d']),
            Jump(2),
            Match
        ]);
    }

    #[test]
    fn test_counter1() {
        let instructions = get_code(&AST::Seq(vec![
            AST::Char('a'),
            AST::Counter(Box::new(AST::Char('d')), (1, Some(3))),
            AST::Char('f'),
        ]))
        .unwrap(); //"ad{3}f"
        assert_eq!(instructions, vec![
            Char('a'),
            Split(2, 5, (1, Some(3)), 0),
            Descrement(0),
            Char('d'),
            Jump(1),
            Char('f'),
            Match,
        ]);
    }

    #[test]
    fn test_capcher() {
        let instructions = get_code(&AST::Seq(
            vec![
                AST::Char('a'),
                AST::Char('b'),
                AST::Chapcher(
                    Box::new(AST::Seq(vec![
                        AST::Chapcher(Box::new(AST::Seq(vec![
                            AST::Counter(
                                Box::new(AST::AnyNumber),
                                (2, Some(2))
                            )
                        ]))),
                        AST::Char('-'),
                        AST::Chapcher(Box::new(AST::Seq(vec![
                            AST::Counter(
                                Box::new(AST::AnyNumber),
                                (2, Some(2))
                            )
                        ]))),
                    ]))
                )
            ]
        ))
        .unwrap(); //"ab((\\d{2})-(\\d{2}))"
        assert_eq!(instructions, vec![
            Char('a'),
            Char('b'),
            CapcherBegin(0),
            CapcherBegin(1),
            Split(5, 8, (2, Some(2)), 0),
            Descrement(0),
            AnyNumber,
            Jump(4),
            CapcherEnd(1),
            Char('-'),
            CapcherBegin(2),
            Split(12, 15, (2, Some(2)), 1),
            Descrement(1),
            AnyNumber,
            Jump(11),
            CapcherEnd(2),
            CapcherEnd(0),
            Match,
        ]);
    }
}
