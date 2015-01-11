/*
	Find the sum of all multiples of 3 or 5 below 1000
*/


fn is_multiple(num: i32) -> bool {
	num % 5 == 0 || num % 3 == 0
}

fn main() {
	let mut sum_of_multiples = 0; 

	//loop from 0..999
	for i in range(0,1000) {
		sum_of_multiples += 
			if is_multiple(i) {
				i
			}else{
				0
			};
	}
	println!("Sum is {}", sum_of_multiples);

}
