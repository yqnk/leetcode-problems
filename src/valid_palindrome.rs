struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().map(|c| c.to_ascii_lowercase()).collect();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if !chars[i].is_alphanumeric() {
                i += 1;
                continue;
            }
            if !chars[j].is_alphanumeric() {
                j -= 1;
                continue;
            }
            if chars[i] != chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

fn main() {
    let word = "A man, a plan, a canal: Panama";
    let result = Solution::is_palindrome(word.to_string());
    println!("The result is {}", result);
}