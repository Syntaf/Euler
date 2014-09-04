/*
	Find the difference between the sum of squares of the first one
	hundred natural numbers and the square of the sum
*/

fn sum_of_squares() -> i64 {
	let mut sum: i64 = 0;
	for i in range(1i,101) {
		sum = sum + (i * i) as i64;
	}
	sum
}

fn square_of_sum() -> i64 {
	let mut sum: i64 = 0;
	for i in range(1i,101) {
		sum = sum + i as i64;
	}
	sum * sum
}

fn main() {
	//this value may be negative, we need to make sure it is a difference
	let difference: i64 = std::num::abs(sum_of_squares() - square_of_sum());
	println! ("The difference is: {}", difference)
}
