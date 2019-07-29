fn main() {
    println!("Hello, world!");
    Solution::trap(vec![0, 1, 0, 1]);
}

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut left_index = 0;
        let mut right_index = height.len() - 1;
        let mut left_h;
        let mut right_h;

        let mut left_total = 0;
        let mut right_total = 0;

        let mut total_water = 0;
        let mut left_max_h= height[left_index];
        let mut left_max_index= left_index;
        let mut right_max_h= height[right_index];
        let mut right_max_index= right_index;

        let mut on_left = !(left_max_h > right_max_h);
        left_index = left_max_index + 1;
        right_index = right_max_index - 1;

        while left_index <= right_index {
            if on_left {
                left_h = height[left_index];
                if left_h >= left_max_h {
                    total_water +=
                        left_total + (left_index - left_max_index - 1) as i32 * left_max_h;
                    left_total = 0;
                    left_max_h = left_h;
                    left_max_index = left_index;
                    left_index += 1;
                } else {
                    left_total -= left_h;
                    left_index += 1;
                    continue;
                }
            } else {
                // on right
                right_h = height[right_index];
                if right_h >= right_max_h {
                    total_water +=
                        right_total + (right_max_index - right_index -1) as i32 * right_max_h;
                    right_total = 0;
                    right_max_h = right_h;
                    right_max_index = right_index;
                    right_index -= 1;
                } else {
                    right_total -= right_h;
                    right_index -= 1;
                    continue;
                }
            }

            on_left = !(left_max_h > right_max_h);
        }
        if on_left {
            total_water += left_total + (right_max_index - left_max_index - 1) as i32 * left_max_h;
        } else {
            total_water += right_total + (right_max_index - left_max_index - 1) as i32 * right_max_h;
        }
        total_water
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 1]), 1);
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(Solution::trap(vec![2, 0, 2]), 2);
    }
}
