fn main() {
    println!("Hello, world!");
}

fn kmp_jump_table(s: &str) -> Vec<usize> {
    let mut jump_table = Vec::new();
    jump_table.push(0);
    let bs = s.as_bytes();
    let len = bs.len();
    let mut index_n = 1;
    let mut index_m = 0;
    
    while index_n < len {
        if bs[index_n] == bs[index_m] {
            jump_table.push(index_m+1);
            index_m += 1;
            index_n += 1;
        }
        else {
            if index_m == 0 {
                jump_table.push(0);
                index_n += 1;
            }
            else {
                index_m = jump_table[index_m-1];
            }
        }
    }
    jump_table
}


#[cfg(test)]
mod test {
    #[test]
    fn test_kmp() {
        assert_eq!(super::kmp_jump_table("acabacacd"), vec![0, 0, 1, 0, 1, 2, 3, 2, 0]);
    }
}