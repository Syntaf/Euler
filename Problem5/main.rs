/*
	What is the smallest number evenly divisible by all of the numbers
	1 to 20?
*/

fn is_divisible_by(dividend: int, divisor: int) -> bool {
	dividend % divisor == 0
}

fn main() {
	let mut found = false;
	let mut answer = 22;
	loop{
		for i in range(1,21) {
			if !is_divisible_by(answer,i) {
				break;
			}else if i == 20 {
				found = true;
				break;
			}
		}
		if found {
			break;
		}
		answer += 1;
	}
	println!("The answer is: {}", answer);
}
