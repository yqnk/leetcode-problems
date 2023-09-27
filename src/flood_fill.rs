struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut stack = vec![(sr as usize, sc as usize)];
        while let Some((r, c)) = stack.pop() {
            if image[r][c] == color {
                continue;
            }
            let old_color = image[r][c];
            image[r][c] = color;
            if r > 0 && image[r - 1][c] == old_color {
                stack.push((r - 1, c));
            }
            if r < image.len() - 1 && image[r + 1][c] == old_color {
                stack.push((r + 1, c));
            }
            if c > 0 && image[r][c - 1] == old_color {
                stack.push((r, c - 1));
            }
            if c < image[0].len() - 1 && image[r][c + 1] == old_color {
                stack.push((r, c + 1));
            }
        }
        image
    }
}

fn main() {
    println!("Hello World!");
}
