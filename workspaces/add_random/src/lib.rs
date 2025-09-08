//! # Add Random
//! 
//! A module for adding a random number to your numbers. (very useful I know)

use rand;

/// Adds a random number between 1 and 10 to the number provided
pub fn add_random(num: u64) -> u64 {
    num + rand::random::<u64>() % 10 + 1
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
