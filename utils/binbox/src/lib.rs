use std::process::exit;

use basename::basename_cli;
use r#false::r#false;
use r#true::r#true;

pub fn bb_basename(mut args: Vec<String>) {
	args.drain(0..1);
	exit(basename_cli(args));
}

pub fn bb_false() {
	exit(r#false());
}

pub fn bb_true() {
	exit(r#true());
}
