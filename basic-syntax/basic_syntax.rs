fn main(){
	// Boolean data type
	let _t = true;

	// char data type
	let _a = 'a'; 
	let _keyboard = '⌨'; 

	// both signed and unsigned data types 
	let _x = 5; // i32 by default
	let _y: u64 = 293402358; // 64-bit unsigned integer (explicit declaration)

	// floating point data type
	let _pi = 3.14159265358; // f64 by default

	// arrays 
    println!("\nARRAYS:");
	let fruits: [&str; 4] = ["Strawberry", "Banana", "Mango", "Watermelon"]; // Array
	println!("The first element of the array is: {}", fruits[0]);

	let mut counter = 0;
	for elem in fruits.iter(){
	    println!("The element at index {} is {}", counter, elem);
	    counter += 1;
	}

	// slices
    println!("\nSLICES:");
	let slice = &fruits[0..2]; // Remember upper bound is exclusive so this will select the first two elements of the fruit array

	for elem in slice {
		println!("elem is {}", elem);
	}

	// str
    println!("\nSTR:");
	let str_slice = "Hello! I'm a str";

	println!("The value of our str is: {}", str_slice);

	// tuple
    println!("\nTUPLE:");
	let tuple = ("hello", 42, "world", [3,6,9]);

	println!("First element is {}", tuple.0);
	println!("Second element is {}", tuple.1);
	println!("Third element is {}", tuple.2);

	let mut counter = 0;
	for elem in &tuple.3 {
	    println!("Element {} of the fourth element is {}", counter, elem);
	    counter += 1;
	}

    //if expression
    println!("\nIF-EXPRESSION:");
    let number = 6;

    if number % 4 == 0 {
      println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //loop
    println!("\nLOOP:");
    let mut loop_x = 5;

    loop{
      loop_x += loop_x -3;

      println!("{}", loop_x);

      if loop_x % 5 == 0 {
          break;
      }
    }

    //countdown - with while
    println!("\nWHILE LOOP:");
    let mut num_w = 3;

    while num_w != 0 {
    print!("{}..", num_w);

    num_w -= 1;
    }
    println!("LIFTOFF!");

    println!("\nFOR LOOP:");
    for num_f in (1..4).rev() {
        print!("{}..", num_f);
    }
    println!("LIFTOFF!\n");
}
