/*
	Find the largest palindrome made from the product of two 3digit numbers
*/

fn is_palindromic(num: int) -> bool {
	let num_str = num.to_str();
	
	if std::iter::order::equals(num_str.as_slice().chars(),
							num_str.as_slice().chars().rev()){
		return true;
	}else{
		return false;
	}
	
}

fn main() {
	let mut max_product =0;
	for i in range(0,999){
		for j in range(0,999){
			if is_palindromic(i*j) && i*j > max_product {
				max_product = i*j;
			}
		}
	}

	println!("The max product is: {}",max_product);
}
