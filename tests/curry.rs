#![feature(type_alias_impl_trait, min_type_alias_impl_trait)]

#[masala::curry]
fn kestrel(a: isize, _b: isize) -> isize {
    a
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
