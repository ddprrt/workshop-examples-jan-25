mod print;

pub use print::Print;

/// This function adds two numbers
/// ```
/// use cargotestlib::add;
/// let x = add(10, 20);
/// assert_eq!(x, 30);
/// ```
pub fn add(left: u128, right: u128) -> u128 {
    left.print();
    right.print();
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
