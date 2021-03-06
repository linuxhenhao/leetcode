use std::str::from_utf8;

fn main() {
    println!("Hello, world!");
    Solution::convert("ABCDE".to_string(), 4);
}

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || s.len() <= num_rows {
            return s;
        }
        let iters = s.len()/(2*(num_rows as usize-1));
        let s_bytes = s.as_bytes();
        let mut result: Vec<u8> = Vec::new();
        let mut offset;
        let regular_int = 2*num_rows-2;
        for r in 0..num_rows {
            for k in 0..iters {
                offset = k*regular_int;
                result.push(s_bytes[r+offset]);
                if r != 0 && r != num_rows - 1 {
                    result.push(s_bytes[offset+regular_int-r]);
                }
            }
            // deal with last iter
            offset = iters*regular_int;
            if r+offset < s.len() {
                result.push(s_bytes[r+offset]);
            }
            let zag = offset+regular_int-r;
            if (r != 0 && r != num_rows - 1) &&
            zag < s.len() {
                result.push(s_bytes[zag]);
            }
        }
        from_utf8(&result).unwrap().to_string()
    }
}


#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert("ABC".to_string(), 1),
            "ABC".to_string()
        );
    }
}