use std::env;
use std::process::exit;

use binbox::{bb_basename, bb_false, bb_true};

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.drain(0..1);

	if args.len() < 1 {
		println!("binbox: error: no utility specified");
		exit(1);
	}

	match args[0].as_str() {
		"basename" => bb_basename(args),
		"false" => bb_false(),
		"true" => bb_true(),
		_ => exit(0),
	}
}
