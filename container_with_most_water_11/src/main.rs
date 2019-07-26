use std::slice::Iter;
use std::iter::Rev;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_h = 0;
        let mut right_h = 0;
        let mut max_area = 0;
        let mut temp_area;
        let mut left_index = 0;
        let mut right_index = height.len() - 1;
        while left_index < right_index {
            left_h = height[left_index];
            right_h = height[right_index];
            if left_h > right_h {
                temp_area = right_h * (right_index - left_index) as i32;
                right_index -= 1;
            } else {
                temp_area = left_h * (right_index - left_index) as i32;
                left_index += 1;
            }

            if temp_area > max_area {
                max_area = temp_area;
            }
        }
        max_area
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 7, 10]), 7);
        assert_eq!(Solution::max_area(vec![1, 7, 3, 10]), 14);
    }
}
