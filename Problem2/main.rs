/*
	Find the sum of the even-valued terms of the fibonnaci sequence
	whose values do not exceed four million
*/


/* calculates the nth digit of the fibonacci sequence specified by term */
fn fibonacci(term: int, value: int, prev: int) -> int {
	match term {
		0		=> prev,
		1		=> value,
		_		=> fibonacci(term-1, value+prev, value)
	}

}

fn main() {
	let mut i = 0;
	let mut sum = 0;
	loop {
		let value = fibonacci(i,1,0);
		if value > 4000000 {
			break;
		}
		match value%2 {
			0		=> sum += value,
			_		=> sum += 0 
		}
		i += 1;
	}
	println!("The total sum of even numbers is: {}", sum);
}
