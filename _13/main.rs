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
    // any none values and unwrap the result into a BigNum
    let numbers: Vec<bignum::BigNum> = reader.lines()
        .filter_map(|x| match x {
            Ok(y) => { Some(bignum::BigNum::new(&y)) },
            _     => { None}
        }).collect();

    println!("{}", &numbers[0] + &numbers[1]);
}
