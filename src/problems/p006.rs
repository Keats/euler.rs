use std::iter::AdditiveIterator;

fn print_question() {
	println!("Problem 6");
	println!("Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.");
}

pub fn solve() {
	print_question();

	let sum_of_squares = range(1i, 101).map(|a| a * a).sum();
	// seems a bit odd that only float have math operations
	let square_of_sum = (range(1i, 101).sum() as f64).powi(2);

	println!("> {}", square_of_sum as int - sum_of_squares);
}
