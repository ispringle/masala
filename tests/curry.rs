#![feature(type_alias_impl_trait, min_type_alias_impl_trait)]

#[masala::curry]
pub fn bluebird<T>(b: fn(T) -> T, c: fn(T) -> T, a: T) -> T {
    b(c(a))
}

#[masala::curry]
fn kestrel<T: Clone>(a: T, _b: T) -> T {
    a.clone()
}

#[masala::curry]
fn add(x: isize, y: isize, z: isize) -> isize {
    x + y + z
}

mod tests {
    use super::*;

    #[test]
    fn test_kestrel() {
        assert_eq!(kestrel(1)(2), 1);
    }

    #[test]
    fn test_add() {
        assert_eq!(6, add(1)(2)(3));
    }
}
