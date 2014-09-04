/*
    Find the sum of all the primes below two million.
*/
fn is_prime(num: i64) -> bool {
  if num <= 1 as i64 {
    false
  } else if num == 2i64 {
    true
  } else if num % 2i64 == 0i64 {
    false
  } else {
    let mut j = 3i64;
    let mut prime = true;
    loop {
      if j*j > num {
        break;
      }
      if num % j == 0 {
        prime = false;
        break;
      }
      j += 1;
    }
    prime
  }
}

fn main() {
  let mut sum = 0i64;
  for i in range(0i64, 2000000) {
    if is_prime(i) {
      sum += i;
    }
  }
  println!("{}",sum);
}
