use std::env;
use std::process::exit;

fn count(args: Vec<String>) -> usize {
    if args.len() == 1 {
        let items: Vec<&str> = args[0].split(':').collect();
        return items.len();
    }

    args.len()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);

    if args.len() < 1 {
        println!("count: error: missing operand");
        exit(1);
    }

    println!("{}", count(args));
}

#[test]
fn count_list_of_items() -> Result<(), String> {
    let items: Vec<String>;
    items = ["wew", "lad"].iter().map(|s| s.to_string()).collect();
    assert_eq!(count(items), 2);
    Ok(())
}

#[test]
fn count_list_of_items_colon_separated() -> Result<(), String> {
    let items: Vec<String>;
    items = ["/bin:/usr/bin"].iter().map(|s| s.to_string()).collect();
    assert_eq!(count(items), 2);
    Ok(())
}
