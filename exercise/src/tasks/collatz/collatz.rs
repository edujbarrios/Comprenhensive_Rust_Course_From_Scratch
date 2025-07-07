/// Calculates the length of the Collatz sequence starting at `n`
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1; // Start counting from the first value (n)
    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15); // Expected length for n = 11
}

fn main() {
    let n = 11;
    let length = collatz_length(n);
    println!("Collatz length for {} is: {}", n, length);
}
