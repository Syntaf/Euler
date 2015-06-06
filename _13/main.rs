// Work out the first ten digit sum of the 100,
// 50-digit numbers in nums.dat

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

/*
fn string_add(op1: &[char], op2: &[char]) -> String {
    let mut ret_str = "this".to_string();
    ret_str.insert(0, op1[0]);
    return ret_str;
}
*/

fn test(u: &[char]) -> bool {
    println!("{}", u[0]);
    true
}

fn main() {
    let file = File::open(&Path::new("nums.dat")).unwrap();
    let reader = BufReader::new(file);

    // read all lines via the buffered reader, then filter
    // any none values and unwrap the result
    let lines: Vec<String> = reader.lines()
        .filter_map(|x| match x {
            Ok(y) => { Some(y) },
            _     => { None}
        }).collect();
    test(&lines[0].chars().collect::<Vec<char>>());
    println!("{}", lines[0].capacity());

    // println!("{}", string_add(Vec::<char>.from_iter(lines[0].chars()), lines[1].chars()));
    // for each line in the vector of lines
    /*
    for line in lines {
        let long_num: u64
        println!("{}", line);
    }
    */
}
