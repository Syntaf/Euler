/*
  find the thirteen adjacent digits in the 1000-digit number that have the greatest
  product. What is the value?

*/

use std::io::File;
use std::io::BufferedReader;

fn read_file(filename: &str) -> String {
  let path = Path::new(filename);
  let mut file = BufferedReader::new(File::open(&path));
  let lines: Vec<String> = file.lines().map(|x|
    x.unwrap()).collect();

  let mut result_string = String::from_str("");
  for ln in lines.iter() {
    result_string.push_str(ln.as_slice());
  }
  return result_string;
}

fn main() {
  let big_digit = read_file("input.dat");
  println!("{}", big_digit);
}
