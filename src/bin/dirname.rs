use std::env;
use std::process::exit;

fn dirname(name: &str) -> &str {
    let mut result: Vec<&str> =
        name.strip_suffix("/").unwrap_or(name).split("/").collect();

    result.sort();
    result.dedup();

    println!("result: '{:?}'", result);

    if result.len() == 1 && result[0] == "" {
        return "/";
    }

    result.last().unwrap()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);

    if args.len() < 1 {
        println!("dirname: error: missing operand");
        exit(1);
    }

    if args.len() > 1 {
        println!("dirname: error: extra operands");
        exit(1);
    };

    let name = &args[0];
    println!("{}", dirname(&name));
    exit(0);
}

#[test]
fn returns_parent_directory() -> Result<(), String> {
    assert_eq!(dirname("/path/to/dir"), "to");
    Ok(())
}

#[test]
fn slashes_return_slash() -> Result<(), String> {
    assert_eq!(dirname("/"), "/");
    assert_eq!(dirname("//"), "/");
    assert_eq!(dirname("///"), "/");
    Ok(())
}

#[test]
fn trailing_slashes_are_trimmed() -> Result<(), String> {
    assert_eq!(dirname("/path/to/dir/"), "to");
    assert_eq!(dirname("/path/to/dir//"), "to");
    assert_eq!(dirname("/path/to/dir///"), "to");
    Ok(())
}
