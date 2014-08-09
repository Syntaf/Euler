/*
    There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    Find the product of abc.
*/

use std::num;

fn find_abc() {
  for a in range(1i,1001) {
    for b in range(1i,1001) {
      for c in range(1i,1001) {
        if a + b + c == 1000 {
          if (num::pow(a,2) + num::pow(b,2) == num::pow(c,2)) &&
            a < b && a < c && b < c {
            println!("a: {}, b: {}, c: {}", a, b, c);
            println!("{}", a * b * c);
            return;
          }
        }
      }
    }
  }
}

fn main() {
  find_abc();
}
