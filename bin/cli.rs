extern crate how;

use std::process::exit;
use how::how;
use std::env;

// how
fn main () {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    usage();
    exit(1);
  }

  let keyword = args[1].to_string();
  how(&keyword);
}

fn usage () {
  let file = include_str!("./usage.txt");
  println!("{}", file);
}
