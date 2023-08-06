pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_arr: [i16; 26] = [0; 26];
        for c in s.chars() {
            s_arr[(c as usize) - 97] += 1;
        }

        for c in t.chars() {
            s_arr[(c as usize) - 97] -= 1;
        }
        for element in s_arr.iter() {
            if *element != 0 {
                return false;
            }
        }
        true
    }
}
fn main() {
    //Test Case 1
    let s = String::from("anagram");
    let t = String::from("nagaram");

    let result = Solution::is_anagram(s.clone(), t.clone());
    println!("is_anagram evaluated to {} for s = {s}, t = {t}", result);

    //Test Case 2
    let s = String::from("rat");
    let t = String::from("car");

    let result = Solution::is_anagram(s.clone(), t.clone());
    println!("is_anagram evaluated to {} for s = {s}, t = {t}", result);
}
