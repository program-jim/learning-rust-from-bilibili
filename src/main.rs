// Functions
// Oct , 2025

// main() ---- Entry Point of whole program
fn main() {
	println!("Hello, Main");
	hello_world();
	tell_height(182);
	say_hello();
	log_human_id("Mike Jirison", 16, 185.0);

	let _X: u32 = {
		let price: u32 = 5;
		let qty: u32 = 10;
		price * qty
	};

	println!("Result is {}", _X);
}

// Hoisting ---- can call function anywhere in your code.
fn hello_world() {
	println!("Hello, Rust");
}

// You can insert input values
fn tell_height(height: u32) {
	println!("My height is {} cm.", height);
}

// Function Test 0
fn log_human_id(name: &str, age: u32, height: f32) {
	println!("My name is {}. I am {} years old, and my height is {} cm.", name, age, height);
}

// Function Test 1
fn say_hello() {
	println!("Hello, Bullshit");
}

// Expressions and Statements
// Expression: Anything that returns a value.
// Statement: Anything that doesn't returns a value.

// Expression Example:
// 5
// true & false
// add(3, 4)
// if condition {value01} else {value02}
// ({code})

fn add(a: i32, b: i32) -> i32 {
	a + b
}

// Note: An function / variables should be written in snake case.
// Snake Case Example: hello_world
// Kebab Case Example: hello-world
