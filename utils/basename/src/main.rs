use std::env;
use std::process::exit;

use basename::basename;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("basename: missing operand");
        exit(1);
    }

    let name = &args[1];
    let suffix: &str;

    if args.len() > 2 {
        suffix = &args[2]
    } else {
        suffix = ""
    }

    let result = basename(&name, suffix);
    println!("{}", result)
}
