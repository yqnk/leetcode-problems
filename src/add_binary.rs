struct Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        a = a.chars().rev().collect();
        b = b.chars().rev().collect();

        let blen = b.len();

        let mut result = a;
        let mut i = 0;
        while i < blen {
            match (result.as_bytes()[i], b.as_bytes()[i]) {
                (b'1', b'1') => {
                    let mut j = i;
                    while j < result.len() && result.as_bytes()[j] == b'1' {
                        j += 1;
                        result.replace_range((j-1)..(j), "0")
                    }
                    if j >= result.len() {
                        result.push('1');
                    } else {
                        result.replace_range((i-1)..(i), "1")
                    }
                },
                (b'0', b'0') => {
                    i += 1;
                    continue;
                },
                _ => result.replace_range((i-1)..(i), "1")
            }
            i += 1;
        }
        result.chars().rev().collect()
    }
}

fn main() {
    println!("{}", Solution::add_binary("1010".to_string(), "1011".to_string()));
}
