
fn print_question() {
	println!("Problem 3");
	println!("What is the largest prime factor of the number 600851475143 ?");
}

fn get_largest_prime_factor(mut n: int) -> int {
	let mut f = 2;

	while f * f <= n {
		if n % f == 0 {
			n = n / f;
		} else {
			f += 1;
		}
	}

	return n;
}

pub fn solve()  {
	print_question();
	println!("> {}", get_largest_prime_factor(600851475143));
}
