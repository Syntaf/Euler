/*
  What is the value of the first triangle number to have over 500 divisors?
*/

use std::iter::range_step_inclusive;

fn calculate_triangle(num: uint) -> uint {
	return ((num + 1) * num) / 2;
}

fn num_divisors(mut n: uint) -> uint {
	let mut divisors = 1u;
	let f = 2u;
	let mut count = 0u;
	while n % f == 0 {
		count += 1u;
		n /= f;
	}
	divisors *= (count + 1);

	for j in range_step_inclusive(3, n, 2) {
		let mut cnt = 0u;
		while n % j == 0 {
			cnt += 1u;
			n /= j;
		}
		divisors *= (cnt + 1);
	}

	return divisors;
}

fn unique_divisors(mut n: uint) -> uint {
	if n % 2 == 0 {
		n /=2;
	}
	return num_divisors(n);
}

fn first_triangle_with_more_than_n_divisors(n: uint) -> uint {
	let mut i = 1u;
	let mut f1 = unique_divisors(i + 1);
	let mut f2 = unique_divisors(i);
	while f1 * f2 <= n {
		f1 = f2;
		f2 = unique_divisors(i + 2);
		i += 1;
	}
	return i;
}	


fn main() {
	let res = first_triangle_with_more_than_n_divisors(500);
	println!("{}", calculate_triangle(res));
}
