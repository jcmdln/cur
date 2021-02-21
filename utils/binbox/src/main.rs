use std::env;
use std::process::exit;

use basename::basename_cli;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.drain(0..1);

	if args.len() < 1 {
		println!("binbox: error: no utility specified");
		exit(1);
	}

	if &args[0] == "basename" {
		args.drain(0..1);
		exit(basename_cli(args));
	}

	exit(0);
}
