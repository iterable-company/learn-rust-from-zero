mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }
    Ok(())
}

fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;
    println!();

    for line in reader.lines() {
        let line = line?;
        exec(expr, &line, true)?;
    }

    Ok(())
}

fn exec(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    for (i, _) in line.char_indices() {
        if engine::do_matching(expr, &line[i..], i, is_depth)? {
            println!("line: {line}, &line[i..]: {:?}, i: {}", &line[i..], i);
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn test_do_matching() {
        assert!(exec("+b", "bbb", true).is_err());
        assert!(exec("*b", "bbb", true).is_err());
        assert!(exec("|b", "bbb", true).is_err());
        assert!(exec("?b", "bbb", true).is_err());

        assert!(exec("abc|def", "def", true).unwrap());
        assert!(exec("(abc)*", "abcabc", true).unwrap());
        assert!(exec("(ab|cd)+", "abcdcd", true).unwrap());
        assert!(exec("abc?", "ab", true).unwrap());
        assert!(exec("abc.e", "abcxe", true).unwrap());
        assert!(exec("^abcd", "abcde", true).unwrap());
        assert!(exec("abcd$", "eabcd", true).unwrap());
        assert!(exec("ab(cd|ef)$", "xyzabef", true).unwrap());
        assert!(exec("ab(cd|ef)$", "abcxyzabef", true).unwrap());
        assert!(exec("a\\d+b", "a012b", true).unwrap());
        assert!(exec("a\\D+b", "acdeb", true).unwrap());
        assert!(exec("ad{2}b", "addb", true).unwrap());
        assert!(exec("ab(cd|ef){3}g", "abcdcdefg", true).unwrap());

        assert!(!exec("abc|def", "efa", true).unwrap());
        assert!(!exec("(ab|cd)+", "", true).unwrap());
        assert!(!exec("abc?", "acb", true).unwrap());
        assert!(!exec("abc.", "aabc", true).unwrap());
        assert!(!exec("^abcd", "babcd", true).unwrap());
        assert!(!exec("abcd$", "abcda", true).unwrap());
        assert!(!exec("ab(c|d)$", "abcd", true).unwrap());
        assert!(!exec("a\\d+b", "acb", true).unwrap());
        assert!(!exec("ad{3}f", "addb", true).unwrap());

        assert!(exec("(abc)*", "aabcabc", true).unwrap());
        assert!(exec("abc", "aabc", true).unwrap());
    }
}
