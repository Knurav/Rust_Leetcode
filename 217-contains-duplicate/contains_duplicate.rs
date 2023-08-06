use std::collections::HashSet;

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
