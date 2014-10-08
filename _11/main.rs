/*
 * find the largest product in a grid going either left
 * right diagonal or vertical
*/

use std::io::File;
use std::io::BufferedReader;

fn read_file(filename: &str) -> Vec<Vec<int>> {
    //open file and read in every line in a vector of strings
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    //this is what we will be returning
    let mut data: Vec<Vec<int>> = Vec::new();
    //iterate over every line
    for ln in lines.iter(){
        //each row that will be added to the vector
        let mut row: Vec<int> = Vec::new();
        //boolean for flagging and adding into our tuple
        let (mut x, mut y) = ('0','0');
        let mut flagged = false;
        //iterator over every char in string, add pairs of chars into one digit
        for ch in ln.as_slice().chars() {
            if ch.is_digit() {
                flagged = !flagged;
                if flagged == true {
                    x = ch;
                } else {
                    y = ch;
                    row.push((x.to_digit(10).unwrap() as int) * 10 +
                             y.to_digit(10).unwrap() as int);
                }
            }
        }
        //add each finished row
        data.push(row);
    }
    
    return data;
}
fn main() {
    let table = read_file("input.dat");
    println!("Table: ");
    for i in table.iter() {
        for j in i.iter() {
            //if value does not have two digits, write
            //zero in front for pretty formatting
            if *j < 10i {
                print!("0");
            }
            //print value
            print!("{} ", j);
        }
        println!("");
    }
}
