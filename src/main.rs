extern crate escape;
use escape::{escape_string};
use std::io::BufRead;

pub fn main() {
  let stdin = std::io::stdin();
  let lock  = stdin.lock();
  for result in lock.lines() {
    let line = result.unwrap();
    println!("\"{}\"", escape_string(line));
  }
}
