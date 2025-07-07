
# When does the Fibonacci function cause a panic?
In this task we were asked to create the fibonacci series, and asked when will the program return a `panic`, here's why:

The recursive version of the Fibonacci function in Rust crafted by myself...

```rust
fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 10;
    println!("Fibonacci({}) = {}", n, fib(n));
}
```

...can cause a **panic at runtime** due to **stack overflow**.

This happens when you call `fib(n)` with a very large value of `n`, like `fib(50_000)` or higher. Since the function uses deep recursion, each call adds a new frame to the stack. Rust has a limited call stack size, and once it's exhausted, the program panics with a stack overflow error.

Important:
- This is not a compile-time error, but itt will compile and run for small values (e.g., `fib(10)` or `fib(30)`).
- For large `n`,  it's better (imo) to use  iterative or memoized version to avoid recursion limits.

