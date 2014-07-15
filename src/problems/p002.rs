use std::iter::AdditiveIterator;


fn print_question() {
	println!("Problem 2");
	println!("By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.");
}

struct Fibonacci {
	prev: int,
	current: int
}

impl Fibonacci {
	fn new() -> Fibonacci {
		Fibonacci {prev: 1, current: 1}
	}
}

impl Iterator<int> for Fibonacci {
	// Needs to implement that function to become an iterator
	fn next(&mut self) -> Option<int> {
		let prev = self.prev;
		self.prev = self.current;
		self.current = prev + self.current;
		return Some(self.prev);
	}

}

pub fn solve() {
	print_question();

	let sum = Fibonacci::new().take_while(|&i| i <= 4000000).filter(|&i| i % 2 == 0).sum();
	println!("> {}", sum);
}

