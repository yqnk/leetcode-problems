struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut element = 0;
        for num in nums.iter() {
            if count > nums.len() / 2 {
                break;
            }
            if count == 0 {
                element = *num;
            }
            match *num {
                _ if element == *num => count += 1,
                _ => count -= 1,
            }
        }
        element
    }
}

fn main() {
    println!("{}", Solution::majority_element(vec![1,2,3,2,2]));
}
