impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while (left < right) {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        -1
    }
}