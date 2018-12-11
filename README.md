# _CSCI3055u Final Project - Rust Programming Language_

- _Mustafa Al-Azzawe_
- _mustafa.alazzawe@uoit.net_

## About the language

### What is Rust?

Rust is a programming language that's focused on safety, speed, and concurrency. Its design lets you create programs that have the performance and control of a low-level language, but with the powerful abstractions of a high-level language. These properties make Rust suitable for programmers who have experience in languages like C and are looking for safer alternative, as well as those from languages like Python who are looking for ways to write code that performs better without sacrificing expressiveness.

Rust is syntactically similar to C and C++, but the big difference is that Rust is _safe by default_; all the memory accesses are checked. It is not possible to corrupt memory by accident.

Rust is a very modern language. It uses LLVM on its backend. Rust supports a mixture of imperative procedural, concurrent actor, object-oriented and pure functional styles. It also helps generic programming and meta programming, in both static and dynamic techniques.

The unifying principles behind Rust are:

- strictly enforcing *safe borrowing* of data
- functions, methods and closures to operate on data
- tuples, structs and enums to aggregate data
- pattern matching to select and destructure data
- traits to define *behaviour* on data

### History 

The language grew out of a personal project started in 2006 by Mozilla employee Graydon Hoare. Mozilla began sponsoring the project in 2009 and announced it in 2010. The same year, work shifted from the initial compiler, which was written in OCaml, to the self-hosting compiler written in Rust (```rustc```).

The first numbered pre-alpha release of the Rust compiler occurred in January 2012. Rust 1.0, the first stable release, was released on May 15, 2015. Following 1.0, stable point releases are delivered every six weeks. 

### Features 

***Seamless compilation and dependency management***
  
  In Rust it is very easy to compile a project and manage dependencies. This is thanks to ```cargo```, compiling, running or testing a project is as simple as a one-lime command.

  ```
  cargo build
  cargo run
  cargo test
  ```

***Easy testing and continuous integration***

  Writing tests in Rust is quite straightforward. All you have to do is to declare a function with ```#[test]```.

  ```Rust
  //function
  fn add(x: i32, y: i32) -> i32 {
    x + y
  }

  //test
  #[test]
  fn test_add() {
    assert_eq! (add(12, 34), 46);
  }
  ```

  Then, it is a simiple as running ```cargo test``` and Corgo will collect all tests, compile and run them.

***Error Messages***

  Error messages are well presented, aware of the context, and suggest potential misspellings.

  ```Rust 
  fn main() {
    let foo = "Hello, world!";
    println!("{}", fool);
  }
  ```

  yields the error message:

  ```
  error[E0425]: cannot find value `fool` in this scope
   --> src/main.rs:3:20
    |
  3 |     println!("{}", fool);
    |                    ^^^^ did you mean `foo`?

  error: aborting due to previous error
  ```

## About the syntax

The syntax of Rust is similar to C and C++, with blocks of code delimited by curly brackets, and control flow such as ```if```, ```else```, ```while```, and ```for```. 

It is a good mix of functional and imperative styles, providing advantages of the former (pattern matching, immutability by default) and the latter (procedural flow easy to follow and close to machine operations).

### Rust by Example

***Hello, World!***

```Rust
//single-line commment
fn main() {
  /*
  multi-line comment
  */
  println!("Hello, World!");
}
```
The exclamation mark indicates that this is a _macro_ call. The ```println!``` macro takes a format string and some values.

***Primitive Types***

  Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.

  Rust is a *statically typed* language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

  *Scalar Types*

  - Signed Intergers: ```i8```, ```i16```, ```i32```, ```i64```, ```i128``` and ```isize``` 
  - Unsigned Intergers: ```u8```, ```u16```, ```u32```, ```u64```, ```u128``` and ```usize```  
  - Floating Point: ```f32```, ```f64```
  - Numeric Operations: ```+```, ```-```, ```*```, ```/```, ```%```
  - Character: ```char```, with unicode scalar values
  - Boolean: ```bool```, either ```true``` or ```false``` 
  - Unit Type: ```()```

  *Compound Types*

  - Arrays: ```[1, 2, 3]```
  - Tuples: ```(1, true)```

  *Custom Types*

  Rust custom data types are formed mainly through the two keywords:

  - ```struct```: define a structure 
  - ```enum```: define an enumeration

  Constants can also be created via the ```const``` and ```static``` keywords


  Values (like literals) can be bound to variables, using the ```let``` binding.

  ```Rust 
  fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
  }
  ```

***Functions***
  
  Rust code uses *snake case* as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

  ```Rust 
  fn main() {
    println!("Hello, world!");

    another_function();
  }

  fn another_function() {
    println!("Another function.");
  }
  ```

  The ```main``` function is the entry point of many programs.

  Function definitions in Rust start with ```fn``` and have a set of parentheses after the function name. The curly brackets tell the compiler where the function body begins and ends.

  *Function - Parameters*

  Functions can also be defined to have *parameters*, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters.

  ```Rust 
  fn main() {
    another_function(5);
  }

  fn another_function(x: i32, y: i32) {
      println!("The value of x is: {}", x);
      println!("The value of y is: {}", y);
  }
  ```

  In function signatures, you *must* declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.

  *Functions - Return Values*

  Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (```->```). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. 

  ```Rust 
  fn five() -> i32 {
    5
  }

  fn main() {
    let x = five();

    println!("The value of x is: {}", x);
  }
  ```

  There are no function calls, macros, or even ```let``` statements in the five function—just the number ```5``` with no semicolon because it's an expresson whose value we want to return. Note that the function’s return type is also specified, as ```-> i32```.

  ```Rust 
  fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
  }

  fn plus_one(x: i32) -> i32 {
    x + 1
  }
  ```

  Running this code will print ```The value of x is: 6```. But if we were to add a semicolon at the end of ```x + 1```, this would change it from an expression to a statement, resulting in a error.

  ```Rust
  fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
  }

  fn plus_one(x: i32) -> i32 {
    x + 1;
  }
  ```

***Control Flow***

  The most common constructs that let you control the flow of execution of Rust code are ```if``` expressions and loops.

  *```if``` Expression*

  The ```if``` expression in Rust is the same as many other languages. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

  ```Rust 
  fn main() {
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
  }
  ```

  In Rust you may also use an ```if``` expression in a ```let``` statement.

  ```Rust 
  fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
  }
  ```

  *Repetition with Loops*

  It’s often useful to execute a block of code more than once. For this task, Rust provides several *loops*.
  
  Rust has three kinds of loops: ```loop```, ```while```, and ```for```. 

  *Repeating Code with ```loop```*

  The ```loop``` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

  ```Rust 
  fn main() {
    loop{
      println!("again!");
    }
  }
  ```

  Running this program will result in ```again!``` being printed over and over until we stop the program manually.

  Fortunately, Rust provides another, more reliable way to break out of a loop. You can place the ```break``` keyword within the loop to tell the program when to stop executing the loop.

  ```Rust 
  fn main() {
    let mut x = 5;

    loop{
        x += x -3;

        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }
  }
  ```

  *Conditional Loops with ```while```*

  It’s often useful for a program to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls ```break```, stopping the loop. This loop type could be implemented using a combination of ```loop```, ```if```, ```else```, and ```break```.

  However, this pattern is so common that Rust has a built-in language construct for it, called a ```while``` loop.
  
  ```Rust 
  //countdown - with while
  fn main() { 
    let mut number = 3;

    while number != 0 {
      print!("{}..", number);

      number -= 1;
    }
    println!("LIFTOFF!");
  }
  ```

  *Looping Through a Collection with ```for```*

  Rust's ```for``` loops work a bit differently than in other languages. 

  C vs Rust ```for``` loop

  ```C
  //C for loop 
  for(x = 0; x < 10; x++) {
    printf("%d\n", x);
  } 
  ```

  ```Rust
  //Rust for loop 
  for x in 0..10 {
    println!("{}", x);
  } 
  ```

  Where's what countdown would look like using a ```for``` loop.

  ```Rust 
  fn main() {
    for number in (1..4).rev() {
      print!("{}..", number);
    }
    println!("LIFTOFF!");
  }
  ```

## About the tools

> - ```rustc``` is the compiler for the Rust Programming language, provided by the project itself. 
> - The compiler takes your source code and produces binary code, either as a library or executable.
> - Most Rust programmers don't invoke ```rustc``` directly, but instead do it through *Cargo*.
> - *Cargo* is a tool that allows Rust packages to declare their various dependencies and ensure that you'll always get a repeatable build.
>
> - To accomplish this, *Cargo* does four things:
>     - introduces two metadata files with various bits of package information.
>     - fetches and builds your packages's dependencies.
>     - invokes ```rustc``` or another build tool with the correct parameters to build your package.
>     - introduces conventions to make working with Rust packages easier


## About the standard library

> The Rust Standard Library is the foundation of portable Rust software, a set of minimal and battle-tested shared abstractions for the broader Rust ecosystem. It offers core types, library-defined operations on language primitives, standard macros, I/O and multithreading, among many other things.
>
>```std``` is available to all Rust crayes by default, just as if each one contained an ```extern crate std;``` import at the crate port. This allows us to access the standard library with ```use``` statements through the path std. (ex. ```use std::env```)
>
>As the Standard Library is quite large, I will only be listing a few examples of each..
>
>*Primitive Types*
> - ```i32```, 32-bit signed integer  
> - ```u32```, 32-bit unsigned integer
> - ```f32```, 32-bit floating point
> - ```char```, a character 
> - ```bool```, boolean
> - ```array```, a fixed-size array
> - ```slice```, a dynamically-sized view into a contiguous sequence
> - ```str```, string slices
>
>*Modules*
> - ```alloc```, memory allocation
> - ```env```, inspection and manipulation of the process's environment
> - ```process```, a module for working with processes
> - ```string```, a UTF-8 encoded, growable string
> - ```thread```, native threads
> - ```vec```, a contiguous growable array type with heap-allocated contents
>
>*Macros*
> - ```assert```, ensure that a boolean expression if ```true``` at runtime
> - ```concat```, concatenates literals into a static string slice
> - ```eprintln```, macro for printing to the standard error, with a newline
>
>*Keywords*
> - ```as```, the keyword for casting a value to a type
> - ```const```, the keyword for defining constants
> - ```extern```, for external connections in Rust code  
> 
>
> A program using The Rust Standard Library: 
>```Rust 
>fn main() {
>    let args: Vec<String> = env::args().collect();
>
>    let config = Config::new(&args).unwrap_or_else(|err| {
>        eprintln!("Problem parsing arguments: {}", err);
>        process::exit(1);
>    });
>
>    if let Err(e) = minigrep::run(config) {
>        eprintln!("Application error: {}", e);
>
>        process::exit(1);
>    }
>}
>```

## About open source library

>**Rocket**
>
> - Rocket is a web framework for Rust taht makes it simple to write *fast*, secure web applications without sacrificing flexibility, usability, or type safety.
> - The official Github repo,  https://github.com/SergioBenitez/Rocket
>
>
> - An example of a complete Rocket application:
> 
>    ```Rust 
>     #![feature(proc_macro_hygiene, decl_macro)]
>
>     #[macro_use] extern crate rocket;
>
>     #[get("/hello/<name>/<age>")]
>     fn hello(name: String, age: u8) -> String {
>       format!("Hello, {} year old named {}!", age, name)
>     }
>
>     fn main() {
>       rocket::ignite().mount("/", routes![hello]).launch();
>     }
>    ```
> - Rocket requires a nightly version of Rust as it makes heavy use of syntax extenstions.
> - All of the Rocket libraries are managed by *Cargo*. Resulting in a simple compilation

# Analysis of the language

> _Organize your report according to the project description
document_.


