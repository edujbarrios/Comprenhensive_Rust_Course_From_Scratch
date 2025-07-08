# Comprenhensive Rust Course From Scratch

*****Refreshing rust concepts with this Google Course, done step by step.*****

This guide will be used to mark key rust aspects, take it as a self develope notebook to write down concepts about **Rust**.

> [!IMPORTANT]
> **IF YOU DECIDE TO USE THIS MATERIAL TO LEARN**: This notebook doesn't contain the integrity of Google's Course, it just has the same structure of the course, but in this case all examples of code in the notebook have been crafted by myself from scratch. **The only similar content are the tasks of each course section.**
## Creating Projects 
1. Creating New project

```rust
Cargo new exercise
```
2. Run project
```rust
Cargo run exercise
```

3. Check project
```rust 
Cargo check exercise
```

4. Compile without execute
```rust
Cargo build exercise
```

5. Release version
```rust
Cargo build --release
```

----
## Creating first code
```rust
// Comment
// fn = Function
// main = entry_point
// println! = string_to_print
fn main() {
    println!("Fichero editado");
}
```
----
## Use of let and mut

Use of `let` to declare variables. By default, variables are **immutable** — you can't change their value once it's set.

```rust
fn main() {
    let name = "Ana";
    println!("Hello, {}", name);

    // This will cause an error because `name` is immutable:
    // name = "Luis";
}
```
Use of `mut`, in addition to `let`, to make a variable mutable (changeable):

```rust
fn main() {
    let mut age = 30;
    println!("Age: {}", age);

    age = 31;
    println!("Updated age: {}", age);
}
```
---
## Basic Built-in Types in Rust

Below are some basic built-in types in Rust along with example literal syntax for each type:

| Type Category             | Types                              | Literal Examples                     |
|---------------------------|-------------------------------------|--------------------------------------|
| **Signed Integers**       | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123_i64`     |
| **Unsigned Integers**     | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10_u16`              |
| **Floating Point Numbers**| `f32`, `f64`                         | `3.14`, `-10.0e20`, `2_f32`          |
| **Unicode Scalar Values** | `char`                               | `'a'`, `'α'`, `'∞'`                  |
| **Booleans**              | `bool`                               | `true`, `false`                      |

## Type Sizes

- `iN`, `uN`, and `fN` are **N bits** wide.
- `isize` and `usize` are the size of a **pointer** on the target platform.
- `char` is **32 bits** wide.
- `bool` is **8 bits** wide.

---

### Notes

- Underscores (`_`) in numbers are **optional** and are used only to improve readability.
  - Example: `1_000` is the same as `1000` or even `10_00`.
  - `123_i64` can also be written as `123i64`.
---
## Arithmetic in Rust

Rust supports all common arithmetic operations:

| Operation | Symbol | Example            | Result |
|-----------|--------|--------------------|--------|
| Addition  | `+`    | `2 + 3`            | `5`    |
| Subtraction | `-`  | `10 - 4`           | `6`    |
| Multiplication | `*` | `3 * 7`         | `21`   |
| Division  | `/`    | `12 / 4`           | `3`    |
| Remainder | `%`    | `10 % 3`           | `1`    |

###  Custom Arithmetic Function

The following Rust function calculates the "interproduct" of three numbers using multiplication and addition:

```rust
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + c;
}

fn main() {
    println!("Result: {}", interproduct(100, 100, 50));
}
```
---
## Type Inference in Rust

Rust is a statically typed language, but it supports **type inference**. This means the compiler can often figure out the type of a variable based on how it’s used — so you don’t always have to write it explicitly.

### Basic Example

```rust
fn main() {
    let a = 5;        // Inferred as i32 (default integer type)
    let b = 3.14;     // Inferred as f64 (default float type)
    
    println!("a: {}, b: {}", a, b);
}
```
---
## If expressions

```rust
fn main() {
    let x = 10;
    let size = if x < 10 { "small" } else { "big" };
    println!("the given number is: {}", size);
}
```
---
## While, Loop and For

````rust
fn main() {
    // ===== WHILE LOOP EXAMPLE =====
    let mut counter = 0;

    println!("Using a while loop:");
    while counter < 5 {
        println!("counter = {}", counter);
        counter += 1; // increment the counter by 1
    }

    // ===== FOR LOOP EXAMPLE =====
    println!("\nUsing a for loop:");

    // This uses a range: 0..5
    // The syntax `0..5` creates a range that starts at 0 and goes up to, but does NOT include, 5
    // So it will loop through: 0, 1, 2, 3, 4
    for i in 0..5 {
        println!("i = {}", i);
    }

    // You can also use inclusive ranges with `..=`
    // For example, 0..=5 would include the number 5

    // ===== FOR LOOP OVER AN ARRAY =====
    let numbers = [10, 20, 30, 40, 50];

    println!("\nIterating over an array:");
    // `.iter()` lets us loop through each element in the array
    for num in numbers.iter() {
        println!("number = {}", num);
    }
}
````
---

## Break, Continue, and specific labels
```rust

fn main() {
    // ===== Control Flow: break, continue, and labels =====

    // Labeling an outer loop using `'outer`
    'outer: for i in 0..5 {
        println!("Outer loop i = {}", i);

        for j in 0..5 {
            if j == 2 {
                println!("  Continue inner loop when j = {}", j);
                continue; // skips the rest of this inner loop iteration
            }

            if i == 3 && j == 3 {
                println!("  Breaking out of the OUTER loop from inner loop");
                break 'outer; // breaks out of the OUTER loop using the label
            }

            println!("  Inner loop j = {}", j);
        }
    }

    println!("Loop finished.");
}
```

## Blocks, Scope, Shadowing and Functions

```rust
fn main() {
    // ===== GLOBAL SCOPE (inside main) =====
    let x = 5;
    println!("Initial x in main = {}", x);

    // ===== START OF A NEW BLOCK (new scope) =====
    {
        let x = x + 1; // SHADOWS outer x
        println!("Shadowed x inside inner block = {}", x);

        let y = 10; // y only exists in this block
        println!("y inside block = {}", y);
    } // END OF BLOCK — y is now out of scope

    // println!("y outside block = {}", y); // ❌ This would cause a compile error

    println!("x in main after block = {}", x); // Still the original x = 5

    // ===== CALLING A FUNCTION =====
    let result = double(x);
    println!("x doubled = {}", result);

    // Shadowing again at function scope level
    let x = result; // shadows previous x
    println!("x shadowed again in main = {}", x);
}

// ===== Function definition =====
fn double(n: i32) -> i32 {
    println!("Inside function: received n = {}", n);
    n * 2
}
```
---

# Rust Macro's
In Rust, a macro is like a powerful code generator. It expands at compile time and can do things like printing, debugging, generating repetitive code, or handling errors, for instance `println!()` is a **Macro**

```rust
fn main() {
    let name = "Alice";
    let age = 30;

    // `println!` is a macro used to print formatted text to the console
    println!("Name: {}, Age: {}", name, age);

    // `dbg!` is a debug macro that prints the value and the source code expression
    let square = dbg!(age * age); // Shows: [src/main.rs:9] age * age = 900

    println!("Age squared = {}", square);

    // Example using `todo!()`:
    // This macro is used to mark code that hasn't been implemented yet.
    // If this line is executed, the program will panic with a "not yet implemented" message.
    if age > 18 {
        todo!("Implement adult handling logic here");
    }

    // Example using `unreachable!()`:
    // This macro asserts that a code path should never be executed.
    // If it runs, it panics to indicate a serious logic error.
    let option = 1;

    match option {
        1 => println!("Option 1 selected"),
        2 => println!("Option 2 selected"),
        _ => unreachable!("Unexpected option value! This should never happen."),
    }
}
```
## Arrays
The arrays work the same way as in other languages, an important thing to mention is that **the positions start by [ 0 ]** so the first position of the array corresponds to the [ 0 ] and the second one belongs to [ 1 ].
```rust
fn main() {
    // Define an array of integers with a fixed size of 5 elements
    let numbers = [10, 20, 30, 40, 50];

    // Accessing elements by index
    println!("First number: {}", numbers[0]); // Prints the first element (10)
    println!("Third number: {}", numbers[2]); // Prints the third element (30)

    // Getting the length of the array
    println!("Array length: {}", numbers.len());

    // Iterating through an array using a loop
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Using dbg! macro to debug-print the whole array
    dbg!(numbers);

    // Defining an array initialized with the same value
    let zeros = [0; 5]; // Array of five zeros
    println!("Zeros array: {:?}", zeros);

    // Handling out-of-bounds access safely with `.get()`
    match numbers.get(10) {
        Some(value) => println!("Value at index 10: {}", value),
        None => println!("No element at index 10."),
    }
}
```

### Array iteration (using for)

Arrays can be iterated using a `for` loop, which is not possible directly with tuples.

```rust
fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];

    // Iterate over each prime number in the array
    for prime in primes {
        // Check that prime is not divisible by any smaller number (basic primality test)
        for i in 2..prime {
            assert_ne!(prime % i, 0); // Asserts that `prime` is not divisible by `i`
        }
        println!("{} is a prime number.", prime);
    }
} 
```

----

## Tuples 
Tuples are fixed-size collections of values that can have different types. They're useful for grouping related data without creating a full struct. Elements in a tuple are accessed by position using dot notation (e.g., tuple.0).

```rust
fn main() {
    // Define a tuple with different types
    let person: (&str, i32, f64) = ("Alice", 30, 65.5);

    // Accessing elements by index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Weight: {} kg", person.2);

    // Destructuring a tuple into individual variables
    let (name, age, weight) = person;
    println!("Destructured -> Name: {}, Age: {}, Weight: {} kg", name, age, weight);

    // Nested tuples are also possible
    let nested = ((1, 2), (3, 4));
    println!("Nested tuple: ({}, {}) and ({}, {})", nested.0.0, nested.0.1, nested.1.0, nested.1.1);

    // Using dbg! macro to print tuple with source info
    dbg!(person);

    // Tuples can be returned from functions
    let result = calculate_dimensions();
    println!("Width: {}, Height: {}", result.0, result.1);
}

// Function that returns a tuple
fn calculate_dimensions() -> (u32, u32) {
    let width = 800;
    let height = 600;
    (width, height)
}

```

# Patterns and Destructuring

This section demonstrates how to extract values from tuples using pattern matching, it is important to note that Rust allows both manual access via indexing and cleaner destructuring using patterns.

```rust
fn main() {
    // Define a tuple with different types
    let person: (&str, i32, f64) = ("Alice", 30, 65.5);

    // Accessing elements by index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Weight: {} kg", person.2);

    // Destructuring a tuple into individual variables
    let (name, age, weight) = person;
    println!("Destructured -> Name: {}, Age: {}, Weight: {} kg", name, age, weight);

    // Nested tuples are also possible
    let nested = ((1, 2), (3, 4));
    println!("Nested tuple: ({}, {}) and ({}, {})", nested.0.0, nested.0.1, nested.1.0, nested.1.1);

    // Using dbg! macro to print tuple with source info
    dbg!(person);

    // Tuples can be returned from functions
    let result = calculate_dimensions();
    println!("Width: {}, Height: {}", result.0, result.1);

    // === Patterns and Destructuring Examples ===

    let pair = (10, 20);

    // Manual access using dot notation
    let left = pair.0;
    let right = pair.1;
    println!("Manual access -> left: {}, right: {}", left, right);

    // Destructuring using a pattern
    let (left_d, right_d) = pair;
    println!("Destructured -> left: {}, right: {}", left_d, right_d);
}

// Function that returns a tuple
fn calculate_dimensions() -> (u32, u32) {
    let width = 800;
    let height = 600;
    (width, height)
}
```