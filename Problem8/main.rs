/*
  find the thirteen adjacent digits in the 1000-digit number that have the greatest
  product. What is the value?
*/

use std::io::File;
use std::io::BufferedReader;
use std::char::to_digit;

fn read_file(filename: &str) -> String {
  let path = Path::new(filename);
  let mut file = BufferedReader::new(File::open(&path));
  let lines: Vec<String> = file.lines().map(|x|
    x.unwrap()).collect();

  let mut st = String::from_str("");
  for ln in lines.iter() {
    st.push_str(ln.as_slice().slice_to(ln.len()-1));
  }

  let mut result_string = String::from_str("");
  let st_asc = st.into_ascii();
  for i in range(0u, st_asc.len()) {
    if st_asc[i].is_digit() {
      result_string.push_char(st_asc[i].to_char());
    }
  }

  return result_string;
}

fn main() {
  let big_digit = read_file("input.dat").into_ascii();
  let mut max_value: i64 = 0;
  let mut final_vals = vec![0u];

  for i in range(0u,big_digit.len()-12) {
    final_vals.clear();
    let mut temp_value: i64 = 1;
    for c in range(i,i+13) {
      if big_digit[c].to_char().is_digit() {
        temp_value = temp_value * to_digit(big_digit[c].to_char(),10).unwrap() as i64;
        final_vals.push(to_digit(big_digit[c].to_char(),10).unwrap());
      } else {
        println!("{} is not", big_digit[c]);
      }
    }
    if temp_value > max_value {
      max_value = temp_value;
    }
  }
  println!("{}", max_value);

}
