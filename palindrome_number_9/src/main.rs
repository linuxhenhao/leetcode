fn main() {
    println!("Hello, world!");
    Solution::is_palindrome(1410110141);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            x if x < 0 || (x % 10 == 0 && x != 0) => false,
            _ => {
                let mut reverted = 0;
                let mut mutx = x;
                while mutx > reverted {
                    reverted = reverted*10 + mutx%10;
                    mutx /= 10;
                }
                return mutx == reverted || mutx == reverted/10
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(-123), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1), true);
        assert_eq!(Solution::is_palindrome(312), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}