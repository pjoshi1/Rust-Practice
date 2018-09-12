 /*

Find the Fifth root of the sum of the squares of the first 100 ODD numbers only.


 */

 use std::f32;



  fn main() {
 	
 let sum = sq_of_odd(100);

 let new_sum = sum as f32;

 let result: f32= new_sum.powf(0.2);

 println!("{}", result);


 }



 fn sq_of_odd(x: i32) -> i32 {

 	let mut sum =0;

 	for mut i in 1..x{
 		
 		sum +=  i*i ; 
 		i = i+2 ; 

 	}

 	sum

 	
 	}

 	
 