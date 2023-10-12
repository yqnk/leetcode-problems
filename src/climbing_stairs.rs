struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // Same as Fibonacci sequence
        let mut a: i32 = 0;
        let mut b: i32 = 1;
        for i in 0..n {
            let tmp = a;
            a = b;
            b += tmp;
        }
        return b;
    }
}

fn main() {
    for i in 0..10 {
        println!("{} -> {}", i, Solution::climb_stairs(i));
    }
}

// 1 -> [1] 1
// 2 -> [1,1] [2] 2
// 3 -> [1,1,1] [1,2] [2,1] 3
// 4 -> [1,1,1,1] [2,2] [1,1,2], [2,1,1] [1,2,1] 5
// 5 -> [1,1,1,1,1] [1,1,1,2] [1,1,2,1,] [1,2,1,1] [2,1,1,1] [2,2,1] [1,2,2] [2,1,2] 8