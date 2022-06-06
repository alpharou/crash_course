pub fn run() {

	let x: i32 = 5;

	//x = 6; //This assignment is invalid

	println!("x: {}", x);

	let mut y: i32 = 34;

	println!("First value of mutable variable y: {}", y);

	y = 89; //This line creates a warning,
	//Rust compiler understands that previous value
	//was not used before reassigning the variable

	println!("Second value of mutable variable y: {}", y);

}