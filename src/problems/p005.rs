
fn print_question() {
	println!("Problem 5");
	println!("What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?");
}

pub fn solve() {
	print_question();

	println!("> {}", range(20i, 2147483647).find(|a| range(1i, 20).all(|b| a % b == 0)).unwrap());
}
