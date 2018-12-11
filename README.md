# _CSCI3055u Final Project - Rust Programming Language_

- _Mustafa Al-Azzawe_
- _mustafa.alazzawe@uoit.net_

## About the language

### What is Rust?

Rust is a systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using Garbage Collection.

Rust is syntactically similar to C and C++, but the big difference is that Rust is _safe by default_; all the memory accesses are checked. It is not possible to corrupt memory by accident.

Rust is a very modern language. It uses LLVM on its backend. Rust supports a mixture of imperative procedural, concurrent actor, object-oriented and pure functional styles. It also helps generic programming and meta programming, in both static and dynamic techniques.

The unifying principles behind Rust are:

- strictly enforcing **_safe borrowing_** of data
- functions, methods and closures to operate on data
- tuples, structs and enums to aggregate data
- pattern matching to select and destructure data
- traits to define **_behaviour_** on data

### History 

The language grew out of a personal project started in 2006 by Mozilla employee Graydon Hoare. Mozilla began sponsoring the project in 2009 and announced it in 2010. The same year, work shifted from the initial compiler, which was written in OCaml, to the self-hosting compiler written in Rust (```rustc```).

The first numbered pre-alpha release of the Rust compiler occurred in January 2012. Rust 1.0, the first stable release, was released on May 15, 2015. Following 1.0, stable point releases are delivered every six weeks. 

### Features 

####Seamless compilation and dependency management
  
  In Rust it is very easy to compile a project and manage dependencies. This is thanks to ```cargo```, compiling, running or testing a project is as simple as a one-lime command.

  ```
  cargo build
  cargo run
  cargo test
  ```

## About the syntax

The syntax of Rust is similar to C and C++, with blocks of code delimited by curly brackets, and control flow such as ```if```, ```else```, ```while```, and ```for```.

*Hello, World!*

```Rust
fn main(){
  println!("Hello, World!");
}
```
The exclamation mark indicates that this is a _macro_ call. The ```println!``` macro takes a format string and some values; it's very similar to the formatting used by Python 3.

**

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
