extern crate bignum;
use bignum::inits::Zero;
use bignum::BigNum;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let file = File::open(&Path::new("data/data.dat")).unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<BigNum> = reader.lines().
        filter_map(|x| match x {
            Ok(y)  => {Some(BigNum::from_str(&y).unwrap())},
            Err(e) => {panic!(e)}
        }).collect();

    let mut result = BigNum::new(Zero::zero());
    for datum in numbers { 
        result = &result + &datum;
    }

    println!("The total sum of all one hundred numbers: {}", result);
}
