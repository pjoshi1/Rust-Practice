

use std::io;


fn main(){
//let  input = 10;

let mut input = String::new();


println!("Please input a number");
io::stdin().read_line(&mut input).unwrap();
let input_num: i32 = input.trim().parse().unwrap();



let result = fibonnaci_upto(input_num);

println!("The fibonnaci numbers before {} are {}",input_num, result);

}

fn fibonnaci_upto(x: i32){
	
	// calculate fib and make sure its less than n

	let mut i = 1;

	while fibonnaci(i) < x{
		print!(" {} ", fibonnaci(i) );
		i += 1;
	}
}



fn fibonnaci(x: i32) -> i32{

 if x == 1 || x ==2{
 	return 1; 
 }

 fibonnaci(x-1) + fibonnaci(x-2)


}