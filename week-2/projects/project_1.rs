fn main() {
	//my code
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;
	//interest
	let a = p * (1.0 + (r / 100.0)).powf(n);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Therefore, Compound Interest is {}", ci)
}