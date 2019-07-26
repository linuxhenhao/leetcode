fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = height.len() - 1;
        let mut left_h;
        let mut right_h;

        let mut left_total = 0;
        let mut right_total = 0;

        let mut total_water = 0;
        let mut left_max_h;
        let mut left_max_index;
        let mut right_max_h;
        let mut right_max_index;

        let result = Solution::find_next_max(&height, left_index, true);
        left_max_h = result.0;
        left_max_index = result.1;
        let result = Solution::find_next_max(&height, right_index, false);
        right_max_h = result.0;
        right_max_index = result.1;

        let mut on_left = !(left_max_h > right_max_h);
        if left_max_index == right_max_index {
            return 0;
        }
        left_index = left_max_index+1;
        right_index = right_max_index-1;

        while left_index < right_index {
            if on_left {
                left_h = height[left_index];
                if left_h >= left_max_h {

                    if left_index != left_max_index + 1 {
                        // not just after max_h, should caculate water
                        total_water +=
                            left_total + (left_index - left_max_index) as i32 * left_max_h;
                    }

                    let result = Solution::find_next_max(&height, left_index, true);
                    left_max_h = result.0;
                    left_max_index = result.1;
                    left_index = left_max_index + 1;
                } else {
                    left_total -= left_h;
                    left_index += 1;
                    continue;
                }
            } else {
                // on right
                right_h = height[right_index];
                if right_h >= right_max_h {

                    if right_index != right_max_index + 1 {
                        // not just after max_h, should caculate water
                        total_water +=
                            right_total + (right_max_index - right_index) as i32 * right_max_h;
                    }

                    right_max_h = right_h;
                    right_max_index = right_index;

                    let result = Solution::find_next_max(&height, left_index, true);
                    right_max_h = result.0;
                    right_max_index = result.1;
                    right_index = right_max_index + 1;
                } else {
                    right_total -= right_h;
                    right_index += 1;
                    continue;
                }
            }

            on_left = !(left_max_h > right_max_h);
        }
        total_water
    }

    pub fn find_next_max(height: &Vec<i32>, mut index: usize, on_left: bool) -> (i32, usize) {
        let mut max_h = height[index];
        let mut max_index = index;

        let mut cur_h;
        let next_index;
        let is_end;
        if on_left {
            next_index = Box::new(|i| { i+1 });
            is_end = Box::new(|i| { i == height.len() });
        } else {
            next_index = Box::new(|i| { i-1 });
            is_end = Box::new(|i| { i == 0 });
        }
        index = next_index(index);
        while !is_end(index) {
            cur_h = height[index];
            if cur_h < max_h {
                break;
            } else {
                max_h = cur_h;
                max_index = index;
                index = next_index(index);
            }
        }
        (max_h, max_index)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 1]), 1);
    }

    #[test]
    fn test_find_next_max() {
        let v = vec![0, 1, 2, 3];
        assert_eq!(
            Solution::find_next_max(&v, 0, true),
            (3, 3)
        );
    }
}
