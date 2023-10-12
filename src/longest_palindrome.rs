use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            match map.get(&c) {
                Some(count) => { 
                    map.insert(c, count + 1); }
                None => { map.insert(c, 1); }
            }
        }
        let mut count = 0;
        let mut odd = false;
        for (_, value) in map {
            if value % 2 == 0 {
                count += value;
            } else {
                count += value - 1;
                odd = true;
            }
        }
        if odd {
            count += 1;
        }
        count
    }
}

fn main() {
    println!("{:?}", Solution::longest_palindrome("cccbb".to_string()));
}
