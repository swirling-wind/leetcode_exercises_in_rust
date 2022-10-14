use crate::Solution;

impl Solution {
    fn find_record(record: &[Option<u8>; 26], index: u8) -> u8 {
        return if record[index as usize] == None || record[index as usize] == Some(index) {
            index
        } else {
            Self::find_record(&record, record[index as usize].unwrap())
        };
    }

    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut record: [Option<u8>; 26] = [None; 26];
        for s in &equations {
            if s.as_bytes()[1] == '=' as u8 {
                record[Self::find_record(&record, s.as_bytes()[0] - 'a' as u8) as usize] =
                    Option::from(Self::find_record(&record, s.as_bytes()[3] - 'a' as u8));
            }
        }
        for s in &equations {
            if s.as_bytes()[1] == '!' as u8 &&
                Self::find_record(&record, s.as_bytes()[0] - 'a' as u8) ==
                    Self::find_record(&record, s.as_bytes()[3] - 'a' as u8) {
                return false;
            }
        }
        return true;
    }
}

mod test {
    #[test]
    #[ignore]
    fn p0990() {
        use crate::Solution;
        let input_0 = vec![String::from("c==c"), String::from("b==d"), String::from("x!=z")];
        assert_eq!(true, Solution::equations_possible(input_0));

        let input_1 = vec![String::from("a==b"), String::from("b!=a")];
        assert_eq!(false, Solution::equations_possible(input_1));
    }
}