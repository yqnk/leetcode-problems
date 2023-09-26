use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut alphabet = [0; 26];

        for i in 0..s.len() {
            alphabet[s.as_bytes()[i] as usize - 97] += 1;
            alphabet[t.as_bytes()[i] as usize - 97] -= 1;
        }

        for i in alphabet.iter() {
            if *i != 0 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let s = "car";
    let t = "rat";
    let result = Solution::is_anagram(s.to_string(), t.to_string());
    println!("The result is {}", result);
}