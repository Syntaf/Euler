/*
	Find the 10 001st prime number
*/

fn is_prime(number: i64) -> bool {
	for i in range(2i64, number) {
		if number % i == 0 && i != number {
			return false;
		}
	}
	return true;
}

fn main() {
	let mut count = 0i;
	let mut n: i64 = 2;  //0 and 1 don't count dangit!
	let mut answer: i64 = 0;
	//while we havn't found the 10001'st prime number
	while count < 10001 {
		if is_prime(n) {
			count+=1;
			answer = n;
		}
		n+=1;
	}
	println!("{}",answer);
}
