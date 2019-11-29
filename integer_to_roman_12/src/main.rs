fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let number: Vec<i32> = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let mut result = String::new();
        let mut count: i32 = 0;
        let mut num = num;
        while num != 0 {
            for (index, unit) in number.iter().enumerate() {
                count = num / unit;
                for _ in 0..count {
                    result.push_str(roman[index]);
                }
                num = num % unit;
            }
        }
        return result;
    }
}

mod test {
    use super::Solution;
    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(10), "X");
        assert_eq!(Solution::int_to_roman(3010), "MMMX");
    }
}
