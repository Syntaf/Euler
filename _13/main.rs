// Work out the first ten digit sum of the 100,
// 50-digit numbers in nums.dat

mod bignum;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

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

    let big = bignum::BigNum::new(lines[0].clone());
    let big_2 = bignum::BigNum::new(lines[1].clone());

    let new_num = &big + &big_2;

    println!("{}", big);

    //for line in lines {        
    //    println!("{}", line);
    //}
}
