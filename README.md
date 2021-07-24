# Masala

An autocurrying macro for Rust.

## Usage

This crate requires nightly:

```rust
use masala::curry;

#[curry]
fn add(a: isize, b: isize) {
    a + b
}

fn main() {
    println!("{}", add(33)(42));
}
```

## TODO

- [ ] Allow for Generics
- [ ] Allow for curry functions to be written similar to:

```rust
#[curry]
fn demo<T>(a: T) -> (b: T) -> (c: T) -> T {
    a + b + c
}
```
