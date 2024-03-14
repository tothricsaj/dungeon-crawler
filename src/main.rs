use std::fs::File;
use std::io::{self, BufRead, Read};

fn main() {
  let mut file = match  File::open("src/test.story") {
    Err(why) => panic!("could not open the and the reason: {}", why),
    Ok(file) => file,
  };

  let mut content = String::new();

  file.read_to_string(&mut content).unwrap();

  println!("{}", content);
}
