

fn find_factors(mut n: u64,mut fctrs: Vec<u64>) {
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

	
	let size = fctrs.len();
	for i in range(0, size) {
		let value = fctrs.get(i);
		println!("{}", value);
	}
	
}

fn main() {
	let mut primeFactors: Vec<u64> = Vec::new();
	let bigNum: u64 = 600851475143;
	find_factors(bigNum, primeFactors);
}

