# Masala

An autocurrying macro for Rust.

## Usage

This crate requires nightly:

```rust
use masala::curry;

#[curry]
fn mul<T: Multiple>(a: T, b: T) -> T {
    a mul b
}

fn flip<T>(a: Fn(T), b: T, t:T) -> T {
    a(c, b)
}

fn main() {
    println!("{}", add(33)(42));
    println!("{}", flip(add)(33)(42));
}
```

## TODO

- [x] Allow for Generics
- [ ] ~Allow for curry functions to be written similar to~
