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

fn exec(expr: &str, line: &str, is_depth: bool) -> Result<(bool, Vec<String>), DynError> {
    for (i, _) in line.char_indices() {
        let (is_match, matched_str) = engine::do_matching(expr, &line[i..], i, is_depth)?;
        if  is_match {
            println!("line: {line}, &line[i..]: {:?}, i: {}", &line[i..], i);
            return Ok((true, matched_str));
        }
    }
    Ok((false, vec![]))
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn test_do_matching() {
        //https://zenn.dev/catminusminus/articles/cfcc54a7ee9133 キャッシュ
        assert!(exec("+b", "bbb", true).is_err());
        assert!(exec("*b", "bbb", true).is_err());
        assert!(exec("|b", "bbb", true).is_err());
        assert!(exec("?b", "bbb", true).is_err());

        assert_eq!(exec("abc|def", "def", true).unwrap(), (true, vec![]));
        assert_eq!(exec("[abc]*", "abcabc", true).unwrap(), (true, vec![]));
        assert_eq!(exec("[ab|cd]+", "abcdcd", true).unwrap(), (true, vec![]));
        assert_eq!(exec("abc?", "ab", true).unwrap(), (true, vec![]));
        assert_eq!(exec("abc.e", "abcxe", true).unwrap(), (true, vec![]));
        assert_eq!(exec("^abcd", "abcde", true).unwrap(), (true, vec![]));
        assert_eq!(exec("abcd$", "eabcd", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]$", "xyzabef", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]$", "abcxyzabef", true).unwrap(), (true, vec![]));
        assert_eq!(exec("a\\d+b", "a012b", true).unwrap(), (true, vec![]));
        assert_eq!(exec("a\\D+b", "acdeb", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ad{2}b", "addb", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]{3}g", "abcdcdefg", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]{3}g{2}h", "abcdcdefggh", true).unwrap(), (true, vec![]));
        assert_eq!(exec("abc{1,3}d", "abccd", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]{1,3}g{2}h", "abcdefggh", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]{2,}g{2}h", "abcdefggh", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[cd|ef]{2,}g{2}h", "abcdefefggh", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[^cd|ef]", "abg", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab[^cd|ef]", "abef", true).unwrap(), (true, vec![]));
        assert_eq!(exec("ab([^cd]{2})", "abef", true).unwrap(), (true, vec!["ef".to_string()]));
        assert_eq!(exec("ab((\\d{2})-(\\d{2}))", "ab12-34", true).unwrap(), (true, vec!["12-34".to_string(), "12".to_string(), "34".to_string()]));

        assert_eq!(exec("abc|def", "efa", true).unwrap(), (false, vec![]));
        assert_eq!(exec("[ab|cd]+", "", true).unwrap(), (false, vec![]));
        assert_eq!(exec("abc?", "acb", true).unwrap(), (false, vec![]));
        assert_eq!(exec("abc.", "aabc", true).unwrap(), (false, vec![]));
        assert_eq!(exec("^abcd", "babcd", true).unwrap(), (false, vec![]));
        assert_eq!(exec("abcd$", "abcda", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ab[c|d]$", "abcd", true).unwrap(), (false, vec![]));
        assert_eq!(exec("a\\d+b", "acb", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ad{3}f", "addf", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ad{3}f", "addddf", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ab[cd|ef]{3}g{2}h", "abcdcdefgggh", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ab[cd|ef]{1,3}g{2}h", "abcdefcdefggh", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ab[cd|ef]{4,}g{2}h", "abcdefcdggh", true).unwrap(), (false, vec![]));
        assert_eq!(exec("ab[^cd|ef]", "abc", true).unwrap(), (false, vec![]));

        assert_eq!(exec("[abc]*", "aabcabc", true).unwrap(), (true, vec![]));
        assert_eq!(exec("abc", "aabc", true).unwrap(), (true, vec![]));
    }
}
