struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bs = s.as_bytes();
        if bs.len() == 0 {
            return "".to_string();
        }

        if bs.len() == 1 {
            return s;
        }
        let mut substrings: [usize; 1000] = [0; 1000];
        let mut substr_size: usize = 0;
        let mut substrings_tmp: [usize; 1000] = [0; 1000];

        let mut ref_substrings = &mut substrings;
        let mut ref_substrings_tmp = &mut substrings_tmp;

        let mut max_len: usize = 0;
        let mut max_start: usize = 0;

        if bs[0] == bs[1] {
            ref_substrings[0] = 0;
            substr_size += 1;
            max_len = 2;
            max_start = 0;
        }


        let mut last_start;
        let mut substring_len;
        let mut substring_tmp_pos = 0;
        let mut tmp;
        for i in 2..bs.len() {
            // extend old Palindromic or drop broken one
            for j in 0..substr_size {
                last_start = ref_substrings[j];
                substring_len = i - last_start;
                if last_start == 0 || bs[last_start-1] != bs[i]{
                    // pop out
                    if substring_len >= max_len {
                        max_len = substring_len;
                        max_start = i - substring_len;
                    }
                    substr_size -= 1;
                }
                else {
                    ref_substrings_tmp[substring_tmp_pos] = last_start - 1;
                    substring_tmp_pos += 1;
                }
            }
            substring_tmp_pos = 0;

            tmp = ref_substrings;
            ref_substrings = ref_substrings_tmp;
            ref_substrings_tmp = tmp;

            // check and add new palindromic
            if bs[i] == bs[i-1] {
                if max_len <= 2 {
                    max_len = 2;
                    max_start = i-1;
                }
                ref_substrings[substr_size] = i-1;
                substr_size += 1;
            }
            if bs[i] == bs[i-2] {
                if max_len <= 3 {
                    max_len = 3;
                    max_start = i-2;
                }
                ref_substrings[substr_size] = i-2;
                substr_size += 1;
            }
        }
        // now, O(n) scan finished, we have a max_len and max_start;
        // But, there may be a bigger len in substrings, we need 
        // another scan
        for i in 0..substr_size {
            if bs.len() - ref_substrings[i] >= max_len {
                max_len = bs.len() - ref_substrings[i];
                max_start = ref_substrings[i];
            }
        }
        if max_len == 0 {
            s[0..1].to_string();
        }
        s[max_start..max_start+max_len].to_string()
    }
} 

fn main() {
    println!("{}", Solution::longest_palindrome("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
.to_string()));
    // println!("{}", Solution::longest_palindrome("aaaaa"
// .to_string()));
    let s = "abcd".to_string();
    assert!(s[0..0].to_string() == "".to_string());
}