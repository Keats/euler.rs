
fn print_question() {
	println!("Problem 4");
	println!("Find the largest palindrome made from the product of two 3-digit numbers.");
}

pub fn solve() {
	print_question();

	let mut palindrome = 0;

	for i in range(100u, 1000) {
		for j in range(100u, 1000) {
			let product = i * j;

			if product <= palindrome {
				continue
			}


			// that was a lot of try and error to find how to actually reverse a string
			if product.to_string() == product.to_string().as_slice().chars().rev().collect() {
				palindrome = product;
			}
		}
	}

	println!("> {}", palindrome);
}
