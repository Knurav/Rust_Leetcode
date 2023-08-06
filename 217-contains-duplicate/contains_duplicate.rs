use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();

        for n in 0..nums.len() {
            if map.contains(&nums[n]) {
                return true;
            } else {
                map.insert(nums[n]);
            }
        }

        false
    }
}

fn main() {
    let test_cases = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 1],
        vec![1, 1, 1, 1],
        vec![],
        vec![10, 20, 30, 20],
    ];

    for nums in test_cases {
        let result = Solution::contains_duplicate(nums.clone()); //To keep LC Function Sig
        println!("Input: {:?} -> Contains Duplicate: {}", nums, result);
    }
}
