use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
  let mut scenes:Vec<String> = vec![];
  let mut commands:Vec<String> = vec![];

  let file = match  File::open("src/test.story") {
    Err(why) => panic!("could not open the and the reason: {}", why),
    Ok(file) => BufReader::new(file).lines(),
  };

  for line in file.flatten() {
    println!("{}", line);
    let first_char:Option<char> = line.chars().nth(0);
    if first_char == Some('@') {
      scenes.push(line);
    } else if first_char == Some('#') {
      commands.push(line);
    }
  }
  
  println!("commands: {:?}", commands);
  println!("scenes: {:?}", scenes);
}
