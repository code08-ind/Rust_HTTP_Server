# Rust

Rust is a blazing fast and memory efficient: With no runtime or garbage collector.

Rust's rich type system and ownership model guarantees memory-safety and thread safety.

Rust is a modern systems programming language. It is memory safe. It has no null values or no pointers

There are no exceptions in Rust. It uses Result enum to handle errors.

It has modern package manager with us.

It has no data races, if our code compiles successfully. Data race is a condition when two or more threads or concurrently access a shared memory location here.

cargo is the package manager for rust and rustup is the toolchain installer for the rust.

We can install the new project for rust by using command: cargo new <project_name>.

Also cargo.toml will have the different dependencies that are installed in our system.

These dependencies are called as crates here.

We can build our project using the command: cargo build

Running the project can be done using cargo run.

cargo install cargo-expand: To install the dependency.

rustup toolchain list: This will list all the toolchains that are installed in our system.

For memory Management, we make use of the:

Stack
Heap
Pointers
Smart Pointers

Stack: It's a special region of the process memory that stores variables created by each function.

For every function call a new stack frame is allocated on top of the current one.

The size of every variable on the stack has to be known at compile time.

When a function exits it's stack frame is released.

We also have a function infinte which we can call using the:

FUNCTION infinite{
    CALL infinite
}

Heap: It's a region of the process memory that is NOT automatically managed.

It has no size restrictions.

It's accessible by any function, anywhere in the program.

Heap allocations are expensive and we should avoid them when possible.

When we allocate a memory to an integer by using pointer, this is done inside HEAP Memory and inside the stack we either store the address of the variable or the variable itself.

Allocation happens by making use of the ALLOCATE keyword.

To prevent memory leak, it is important to deallocate the memory that is allocated in the heap.

Smart Pointers: They will help us to deallocate memory when used it insid ethe heap memory. It is used by using SMART_POINTER 8. Box is a type of smart pointer that we use here.

Cargo expand will help us to expand the macros that are used in our code.

Ownership Rules Of Rust:

1. Each Value In Rust is owned by a variable.
2. When the owner gets out of scope, the value will be deallocated.
3. There can only be one owner at a time.

If the data type is not complex and it is simple and if the data is not stored inside the heap, we can assign it to new variables.

Rust allows us to use variables passed inside the function, we get use the Referencing inside the Rust and that allows us to not use the ownership.\

References are immutable here by default.

We will be making a HTTP Server here. We will be using the HTTP/1.1. HTTP is a layer 7 protocol, we can send it over the TCP. This can be messaged based.

We send a request here, and in return we get the response here. We would be having certain headers and all.

Our server would be having: TCP listener, HTTP listener and Handler. Everything will be running over the single thread, so at a time our browser will be handling only the single process.

We can also specify the values to the enums we will be using.

If we will be having a value inside that will be defined in the enums, all the values defined after it will be having values after that value.

Every File in Rust is treated as a separate module.

In Rust, the `Result` enum is a built-in type that represents the result of an operation that can either succeed or fail. It's commonly used for error handling. The `Result` type has two variants:

1. `Ok(T)`: Represents a successful computation with a value of type `T`.
2. `Err(E)`: Represents a failed computation with an error value of type `E`.

Here's a simple example demonstrating the usage of `Result`:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // ? operator returns Err if there's an error opening the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ? operator returns Err if there's an error reading the file
    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}
```

In this example:
- `read_file_contents` function attempts to read the contents of a file specified by its path.
- It returns a `Result<String, io::Error>`, where `String` represents the file contents if successful, and `io::Error` represents any potential I/O errors.
- In the `main` function, we use a `match` statement to handle the `Result`. If the operation succeeds (`Ok` variant), we print the file contents. If it fails (`Err` variant), we print the error message.

By using the `Result` enum, Rust enforces explicit error handling, which helps developers write more robust and predictable code.

Match is not only an enum but it can also act as the switch statement in Rust.

We should not implement a trait simply, but we should always try implementing TryFrom trait which will offer greater flexibility and provides an equivalent TryInto implementation for free, thanks to a blanket implementation in the standard library.

Tokio: Async run-time for the Rust Programming language. It is best for asyn programming.
