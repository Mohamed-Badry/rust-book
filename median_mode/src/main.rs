use std::collections::HashMap;

fn get_median(nums: &Vec<i32>) -> Option<i32> { 

    let mut nums_copy = nums.clone();
    if !nums_copy.is_sorted() {
        nums_copy.sort();
    }
    let m = nums_copy.len() / 2;
    nums_copy.get(m).copied()
    
}

fn get_mode(nums: &Vec<i32>) -> Option<Vec<i32>> {
    if nums.is_empty() {
        return None;
    }
    let mut counts = HashMap::new();

    for num in nums.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }

    let max_count = counts.values().cloned().max()?;
    let modes: Vec<i32> = counts
        .iter()
        .filter(|&(_, &count)| count == max_count)
        .map(|(&num, _)| *num)
        .collect();

    Some(modes)
}

fn test_both(nums: &Vec<i32>) {
    let median = get_median(nums);
    let mode = get_mode(nums);

    println!("{:?}", nums);
    println!("median: {:?}", median);
    println!("mode: {:?}", mode);
    println!();
}

fn main() {

    let test1 = vec![];
    let test2 = vec![0, 213, 1230, 102, 1039, 0195, 1048];
    let test3 = vec![0, 0 , 0, 0 ,0 , 0, 0];
    let test4 = vec![-12, -49, -2, -194, -29, -194, -94];
    let test5 = vec![0, 0, 1, 1 , 2, 2, 2, 3, 3, 3];
    
    test_both(&test1);
    test_both(&test2);
    test_both(&test3);
    test_both(&test4);
    test_both(&test5);
    
}
