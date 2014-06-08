

fn find_factors(mut n: u64, fctrs: &mut Vec<u64>) {
	let mut z: u64 = 2;

	while z*z <= n {
		match n%z {
			0		=> {fctrs.push(z); n /= z}
			_		=> {z += 1}
		}
	}
	if n > 1 {
		fctrs.push(n);
	}
}

fn main() {
	let mut prime_factors: Vec<u64> = Vec::new();
	let bigNum: u64 = 600851475143;
	find_factors(bigNum, &mut prime_factors);
	
	let size = prime_factors.len();
	for i in range(0, size) {
		let value = prime_factors.get(i);
		println!("{}", value);
	}

}

