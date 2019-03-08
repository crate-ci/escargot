extern crate test_fixture;

fn main() {
    println!("answer = {}", test_fixture::add(40, 2));
    println!("answer = {}", add(40, 2));
}

/// ```rust
/// assert_eq!(test_fixture::add(1, 2), 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_works() {
        assert_eq!(add(2, 2), 4);
    }
}
