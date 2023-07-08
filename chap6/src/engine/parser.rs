use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

#[derive(Debug)]
pub enum AST {
    Char(char),
    UnmatchChars(Vec<char>),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Caret,
    Doller,
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
    Counter(Box<AST>, (usize, Option<usize>)),
    AnyNumber,
    NotNumber,
    Chapcher(Box<AST>),
}

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    InvalidBrace,
    InvalidCaret,
    InvalidRightBracket(usize),
    NoPrev(usize),
    NoRightParen,
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{c}'")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parentheis: post = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen => {
                write!(f, "ParseError: no right parenthesis")
            }
            ParseError::Empty => {
                write!(f, "ParseError: empty expression")
            }
            ParseError::InvalidBrace => {
                write!(f, "ParseError: invalid brace")
            }
            ParseError::InvalidCaret => {
                write!(f, "ParseError: invalid caret")
            }
            ParseError::InvalidRightBracket(pos) => {
                write!(f, "ParseError: invalid bracket: pos = {pos}")
            }
        }
    }
}

impl Error for ParseError {}

fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' | '.' | '^' | '$' => Ok(AST::Char(c)),
        'd' => Ok(AST::AnyNumber),
        'D' => Ok(AST::NotNumber),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

enum PSQ {
    Plus,
    Star,
    Question,
    Counter((usize, Option<usize>)),
}

fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
            PSQ::Counter((lower_count, upper_count)) => {
                AST::Counter(Box::new(prev), (lower_count, upper_count))
            }
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}

fn unmatch_charctors(seq_or: &mut Vec<AST>, seq: &Vec<AST>) -> Result<(), ParseError> {
    let chars = seq
        .iter()
        .map(|ast| match ast {
            AST::Char(c) => Ok(c.to_owned()),
            _ => Err(ParseError::InvalidCaret),
        })
        .collect::<Result<Vec<char>, _>>()?;
    seq_or.push(AST::UnmatchChars(chars));
    Ok(())
}

pub fn parse(expr: &str) -> Result<AST, ParseError> {
    enum ParseState {
        Char,
        Escape,
        Brace,
    }

    let mut seq = Vec::new();
    let mut seq_or = Vec::new();
    let mut stack = Vec::new();
    let mut state = ParseState::Char;
    let mut counter = "".to_string();
    let mut counter_pair = (0, None);
    let mut expect_second_count = false;
    let mut expect_grouping = false;
    let mut expect_except_charactors = false;

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '^' => {
                    if expect_grouping {
                        if seq.len() == 0 {
                            expect_except_charactors = true;
                        } else {
                            return Err(ParseError::InvalidCaret);
                        }
                    } else {
                        seq.push(AST::Caret)
                    }
                }
                '$' => seq.push(AST::Doller),
                '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                '*' => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                '[' => {
                    let prev = take(&mut seq);
                    let prev_or = take(&mut seq_or);
                    expect_grouping = true;
                    stack.push((prev, prev_or))
                }
                ']' => {
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        if !seq.is_empty() {
                            if expect_except_charactors {
                                unmatch_charctors(&mut seq_or, &seq)?;
                                expect_except_charactors = false;
                            } else {
                                seq_or.push(AST::Seq(seq));
                            }
                        }
                        if let Some(ast) = fold_or(seq_or) {
                            prev.push(ast);
                        }
                        seq = prev;
                        seq_or = prev_or;
                        expect_grouping = false;
                    } else {
                        return Err(ParseError::InvalidRightBracket(i));
                    }
                }
                '(' => {
                    let prev = take(&mut seq);
                    let prev_or = take(&mut seq_or);
                    stack.push((prev, prev_or))
                }
                ')' => {
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        if !seq.is_empty() {
                            prev.push(AST::Chapcher(Box::new(AST::Seq(seq))))
                        }
                        seq = prev;
                        seq_or = prev_or;
                    } else {
                        return Err(ParseError::InvalidRightParen(i))
                    }
                }
                '|' => {
                    if seq.is_empty() {
                        return Err(ParseError::NoPrev(i));
                    } else {
                        if expect_except_charactors {
                            unmatch_charctors(&mut seq_or, &seq)?;
                            expect_except_charactors = false;
                        } else {
                            let prev = take(&mut seq);
                            seq_or.push(AST::Seq(prev));
                        }
                    }
                }
                '\\' => state = ParseState::Escape,
                '{' => state = ParseState::Brace,
                _ => seq.push(AST::Char(c)),
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
            ParseState::Brace => {
                if let Some(_) = c.to_digit(10) {
                    counter.push(c);
                } else {
                    if c == ' ' {
                        // nop
                    } else if c == ',' && !expect_second_count {
                        let count = counter.parse::<usize>().unwrap();
                        counter_pair.0 = count;
                        counter = "".to_string();
                        expect_second_count = true;
                    } else if c == '}' {
                        let counter_result = counter.parse::<usize>();
                        if !expect_second_count {
                            match counter_result {
                                Ok(count) => counter_pair = (count, Some(count)),
                                Err(_) => return Err(ParseError::InvalidBrace),
                            }
                        } else {
                            match counter_result {
                                Ok(c) => counter_pair.1 = Some(c),
                                Err(_) => (),
                            }
                        }

                        parse_plus_star_question(&mut seq, PSQ::Counter(counter_pair), i)?;
                        counter = "".to_string();
                        counter_pair = (0, None);
                        state = ParseState::Char;
                    } else {
                        return Err(ParseError::InvalidBrace);
                    }
                }
            }
        }
    }

    if !stack.is_empty() {
        return Err(ParseError::NoRightParen);
    }

    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(ParseError::Empty)
    }
}
