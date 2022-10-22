use crate::Solution;

impl Solution {
    
    pub fn min_window(s: String, t: String) -> String {
        use std::usize::MAX;

        let mut map_for_char: Vec<i32> = vec![0; 128];
        for &ascii in t.as_bytes() {
            map_for_char[ascii as usize] += 1;
        }

        let mut uncovered_char_num: i32 = t.len() as i32;

        let mut str_head: usize = 0;
        let mut str_tail: usize = MAX;

        let mut left: usize = 0;
        let mut right: usize = 0;

        let s_seq = s.as_bytes();
        loop {
            if right >= s.len() {
                break;
            }

            // move right
            if map_for_char[s_seq[right] as usize] > 0 {
                uncovered_char_num -= 1;
            }
            map_for_char[s_seq[right] as usize] -= 1;
            right += 1;

            // check if shrink left
            loop {
                if uncovered_char_num != 0 {
                    break;
                }

                if right - left < str_tail - str_head {
                    (str_head, str_tail) = (left, right);
                }

                // shrink
                map_for_char[s_seq[left] as usize] += 1;
                if map_for_char[s_seq[left] as usize] > 0 {
                    uncovered_char_num += 1;
                }
                left += 1;
            }
        }
        dbg!((str_head, str_tail));
        
        if str_tail < MAX {
            return s.as_str()[str_head..str_tail].to_string();
        } else {
            return String::from("");
        }        
    }
}

mod test {
    #[test]
    fn p0076() {
        use crate::Solution;

        dbg!(Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")));
        // assert_eq!(Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")), String::from("BANC"));
    }
}
