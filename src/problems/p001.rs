// gives us the .sum() method
use std::iter::AdditiveIterator;

fn print_question() {
	println!("Problem 1");
	println!("Find the sum of all the multiples of 3 or 5 below 1000.");
}

pub fn solve()  {
	print_question();

	// Naive
	// let mut sum = 0u;
	// for i in range(0u, 1000) {
	// 	if i % 3 == 0 || i % 5 == 0 {
	// 		sum += i;
	// 	}
	// }
	// println!("> {}", sum);

	let sum = range(0u, 1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum();
	println!("> {}", sum);
}
