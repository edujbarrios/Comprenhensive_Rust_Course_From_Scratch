# Comprenhensive Rust Course From Scratch

Refreshing rust concepts with this Google Course, done step by step.

This guide will be used to mark key rust aspects.

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

## Notes

- Underscores (`_`) in numbers are **optional** and are used only to improve readability.
  - Example: `1_000` is the same as `1000` or even `10_00`.
  - `123_i64` can also be written as `123i64`.