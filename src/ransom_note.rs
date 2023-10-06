struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_alphabet = [0; 26];
        for i in 0..magazine.len() {
            magazine_alphabet[magazine.as_bytes()[i] as usize - 97] += 1;
        }
        println!("{:?}", magazine_alphabet);
        for i in 0..ransom_note.len() {
            let char_pos = ransom_note.as_bytes()[i] as usize - 97;
            if magazine_alphabet[char_pos] > 0 {
                magazine_alphabet[char_pos] -= 1;
            } else {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let ransom_note = "aa";
    let magazine = "aab";
    let result = Solution::can_construct(ransom_note.to_string(), magazine.to_string());
    println!("The result is {}", result);
}