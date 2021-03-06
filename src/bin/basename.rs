use std::env;
use std::process::exit;

fn basename<'n>(name: &'n str, suffix: &str) -> &'n str {
    let result: Vec<&str> = name
        .strip_suffix("/")
        .unwrap_or(name)
        .strip_suffix(suffix)
        .unwrap_or(name)
        .split("/")
        .collect();

    result.last().unwrap()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);

    if args.len() < 1 {
        println!("basename: error: missing operand");
        exit(1);
    }

    if args.len() > 2 {
        println!("basename: error: extra operands");
        exit(1);
    }

    let name = &args[0];
    let suffix: &str;

    if args.len() == 2 {
        suffix = &args[1]
    } else {
        suffix = ""
    }

    println!("{}", basename(&name, suffix));
    exit(0);
}

#[test]
fn suffix_is_empty() -> Result<(), String> {
    assert_eq!(basename("/path/to/dir", ""), "dir");
    Ok(())
}

#[test]
fn suffix_is_invalid() -> Result<(), String> {
    assert_eq!(basename("/path/to/dir", "to"), "dir");
    Ok(())
}

#[test]
fn suffix_is_valid() -> Result<(), String> {
    assert_eq!(basename("/path/to/dir", "r"), "di");
    Ok(())
}

#[test]
fn trailing_slash_is_removed() -> Result<(), String> {
    assert_eq!(basename("/path/to/dir/", "r"), "di");
    Ok(())
}
