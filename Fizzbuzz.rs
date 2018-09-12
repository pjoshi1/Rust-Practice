


 fn main() {
	
for x in 0..100{
	fizbuzz(x)
}
}

fn fizbuzz(x: i32) -> (){

	if x % 15 ==0 {
		println!("FizzBuzz");
	}

	else if x % 3 == 0{
		println!("Fizz");
	}

	else if x % 5 ==0{
		println!("Buzz");
	}

	else {
		println!("{} ", x);
	}
}