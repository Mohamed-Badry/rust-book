/// Adds one to the number provided
///
/// ## Examples
/// ```
/// assert_eq!(5, add_one::add_one(4));
/// ```
pub fn add_one(num: u64) -> u64 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
