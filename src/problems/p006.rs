use std::iter::AdditiveIterator;
use std::num;

fn print_question() {
	println!("Problem 6");
	println!("Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.");
}

pub fn solve() {
	print_question();

	let sum_of_squares = range(1i, 101).map(|a| a * a).sum();
	let square_of_sum = num::pow(range(1i, 101).sum(), 2);

	println!("> {}", square_of_sum - sum_of_squares);
}
