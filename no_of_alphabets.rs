

fn main() {

	let my_string = "Hello";

	println!("Number of Constants in {} is {}", my_string, number_of_a_and_c(my_string));
	
}

enum Alphabets{
	a, 
	e, 
	i, 
	o, 
	u,

}

enum Constants{


	b, c, d, f, g, h, j, k, l , m, n, p, q, r, s, t, v, w, x, y, z, 

}

fn number_of_a_and_c(my_string: String ) -> (i32, i32){

	let mut a = 0;
	let mut c = 0;


	for x in my_string.chars(){
		match x {
		 	Some(Alphabets) => a+1 , 
		 	Some(Constants)=> c+1,
		 	None => None
		 } 
		

		(a, c)
	}

// Iterate through string. 
//If character = a, e, i, o, u = a +
// If character = constant, c+ 


}