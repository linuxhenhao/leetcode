fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut new_x = x;
        let mut current: i32 = 0;
        let mut result: i32 = 0;
        // skip trailing zeros
        while new_x % 10 == 0 {
            new_x /= 10;
        }
        while new_x != 0 {
            current = new_x % 10;
            result = match result.checked_mul(10) {
                Some(i) => i,
                None => return 0,
            };
            result = match result.checked_add(current){
                Some(i) => i,
                None => return 0,
            };
            new_x /= 10;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::reverse(5), 5);
        assert_eq!(Solution::reverse(115), 511);
    }
}