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

fn find_largest_product(data: Vec<Vec<int>>) {
    let mut digits_answer: [int, ..4] = [0,0,0,0];
    let mut max_product = 0;

    //loop through all elements in vector
    for i in range(0u, data.len()) {
        for j in range(0u, data[i].len()) {
            let mut temp_product = 1;

            //all of these algorithms remain the same, but with
            //different parameters. A loop will run 4 times and
            //fill the digit array with the selected 4 digits to
            //test and find the product. If the product is greater
            //max product becomes the current product and then
            //temp product is reset to one(NOT ZERO)

            //test previous 4 elements
            if j - 4 < data[i].len() {
                for h in range(0u, 4) {
                    digits_answer[h] = data[i][j-h];
                    temp_product = temp_product * data[i][j-h];
                }
                if temp_product > max_product {
                    max_product = temp_product;
                }
                temp_product = 1;
            }

            //test next 4 elements
            if j + 4 < data[i].len() {
                for h in range(0u, 4) {
                    digits_answer[h] = data[i][j+h];
                    temp_product = temp_product * data[i][j+h];
                }
                if temp_product > max_product {
                    max_product = temp_product;
                }
                temp_product = 1;
            }

            //test up 4 elements
            if i - 4 < data.len() {
                for h in range(0u, 4) {
                    digits_answer[h] = data[i-h][j];
                    temp_product = temp_product * data[i-h][j];
                }
                if temp_product > max_product {
                    max_product = temp_product;
                }
                temp_product = 1;

                //test diagonal
                if j + 4 < data[i].len() {
                    for h in range(0u, 4) {
                        digits_answer[h] = data[i-h][j+h];
                        temp_product = temp_product * data[i-h][j+h];
                    }
                    if temp_product > max_product {
                        max_product = temp_product;
                    }
                    temp_product = 1;
                }

                if j - 4 < data[i].len() {
                    for h in range(0u, 4) {
                        digits_answer[h] = data[i-h][j-h];
                        temp_product = temp_product * data[i-h][j-h];
                    }
                    if temp_product > max_product {
                        max_product = temp_product;
                    }
                    temp_product = 1;
                }

            }

            //test down 4 elements
            if i + 4 < data.len() {
                for h in range(0u, 4) {
                    digits_answer[h] = data[i+h][j];
                    temp_product = temp_product * data[i+h][j];
                }
                if temp_product > max_product {
                    max_product = temp_product;
                }
                temp_product = 1;

                //test diagonal
                if j + 4 < data[i].len() {
                    for h in range(0u, 4) {
                        digits_answer[h] = data[i+h][j+h];
                        temp_product = temp_product * data[i+h][j+h];
                    }
                    if temp_product > max_product {
                        max_product = temp_product;
                    }
                    temp_product = 1;
                }

                //test opposite diagonal
                if j - 4 < data[i].len() {
                    for h in range(0u, 4) {
                        digits_answer[h] = data[i+h][j-h];
                        temp_product = temp_product * data[i+h][j-h];
                    }
                    if temp_product > max_product {
                        max_product = temp_product;
                    }
                }
            }
        }
    }
    println!("The maximum product is: {}", max_product)
    for h in range(0u, 4) {
      print!("{} * ", digits_answer[h]);
    }
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
    println!("");
    println!("Now finding largest product: ");
    find_largest_product(table);
}
