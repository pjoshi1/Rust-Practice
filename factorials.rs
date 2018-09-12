
// Import standard input 

use std::io ;


 fn main() {

 	println!("Please input a number");

 	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let input_num: i32 = input.trim().parse().unwrap();

	println!("The factorial of {} is {}",input_num, factorial(input_num) );
        


    
}

fn factorial (x: i32) -> i32{

	if x ==1 {
		return 1;
	}

	 x * factorial(x -1)

}