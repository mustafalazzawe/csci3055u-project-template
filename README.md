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

**Seamless compilation and dependency management**
  
  In Rust it is very easy to compile a project and manage dependencies. This is thanks to ```cargo```, compiling, running or testing a project is as simple as a one-lime command.

  ```
  cargo build
  cargo run
  cargo test
  ```

**Easy testing and continuous integration**

  Writing tests in Rust is quite straightforward. All you have to do is to declare a function with ```#[test]```.

  ```Rust
  //function
  fn add(x: i32, y: i32) -> i32{
    x + y
  }

  //test
  #[test]
  fn test_add(){
    assert_eq! (add(12, 34), 46);
  }
  ```

  Then, it is a simiple as running ```cargo test``` and Corgo will collect all tests, compile and run them.

**Error Messages**

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
fn main(){
  /*
  multi-line comment
  */
  println!("Hello, World!");
}
```
The exclamation mark indicates that this is a _macro_ call. The ```println!``` macro takes a format string and some values.

***Data Types***

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






## About the tools

> _Describe the compiler or interpreter needed_.

## About the standard library

> _Give some examples of the functions and data structures
> offered by the standard library_.

## About open source library

> _Describe at least one contribution by the open source
community written in the language._

# Analysis of the language

> _Organize your report according to the project description
document_.


