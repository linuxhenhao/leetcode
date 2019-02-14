fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        if str.len() == 0 {
            return 0;
        }
    
        let mut index = 0;
        let mut result: i32 = 0;
        let mut atom = 1;
        let str_bytes = str.as_bytes();
        let array = ['0' as u8, '1' as u8, '2' as u8,
                     '3' as u8, '4' as u8, '5' as u8,
                     '6' as u8, '7' as u8, '8' as u8,
                     '9' as u8];
        
        while index != str_bytes.len() && str_bytes[index] == ' ' as u8 {
            index += 1;
        }
        if index == str_bytes.len() {
            return 0;
        }
        if str_bytes[index] == '+' as u8 {
            index += 1;
        }
        else {
            if str_bytes[index] == '-' as u8 {
                atom = -1; 
                index += 1;
            }
        }
        if index == str_bytes.len() {
            return 0;
        }
        for i in index..str_bytes.len() {
            if array.contains(&str_bytes[i]) {
                result = match result.checked_mul(10) {
                    Some(i) => i,
                    None => {
                        if atom < 0 {
                            return i32::min_value();
                        }
                        else {
                            return i32::max_value();
                        }
                    }
                };

                result = match result.checked_add((
                    (str_bytes[i] - '0' as u8) as i32)*atom) {
                        Some(i) => i,
                        None => {
                            if atom < 0 {
                                return i32::min_value();
                            }
                            else {
                                return i32::max_value();
                            }
                        },
                    }
            }
            else {
                return result;
            }
        }
        return result;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::my_atoi(" ".to_string()), 0);
    }
}