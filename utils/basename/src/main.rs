use std::env;
use std::process::exit;

use basename::basename_cli;

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.drain(0..1);

	let retval: i32 = basename_cli(args);
	exit(retval);
}
