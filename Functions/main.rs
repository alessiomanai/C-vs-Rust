
fn add_ab(a: u32, b: u32) -> u32 {		//u32 is 32 bit integer value
	
	a + b	//you can return a value with return or omitting semicolon

} 


fn main(){		//main function is called first like C language

	let a = 2;
	let b = 2;

	println!("{}", add_ab(a,b));
	println!("{}", mul_ab(a,b));
}


fn mul_ab(a: u32, b: u32) -> u32{

	return a*b;
}