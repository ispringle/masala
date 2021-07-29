# Masala

An autocurrying macro for Rust.

## Usage

This crate requires nightly:

```rust
use masala::curry;

#[curry]
fn mul<T: Multiple>(a: T, b: T) -> T {
    a * b
}

#[curry]
fn add<T>(a: Fn(T), b: T, t:T) -> T {
    a + b
}

#[curry]
pub fn psi<T: Clone>(a: fn(T, T) -> T, b: fn(T) -> T, c: T, d: T) -> T {
    a(b(c.clone()), b(d))
}

fn main() {
    let add_10 = add(10);
    println!("{}", psi(mul)(add)(10)(3)); // (10 + 10) * (3 + 10) = 33
}
```

## TODO

- [x] Allow for Generics
- [ ] ~Allow for curry functions to be written similar to~
