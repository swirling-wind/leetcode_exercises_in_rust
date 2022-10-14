use crate::Solution;

//TODO: Could this problem be solved by Dynamic programming?
impl Solution {
    // fn get_force_direction(position: usize, str_bytes: &[u8], dp: &Vec<Option<u8>>) -> u8 {
    //     if Vec[position] == None {
    //         if str_bytes[position] == '.' as u8 {
    //             //TODO: What is the recursion formula?
    //         }
    //         else {
    //             dp[position] = Option(str_bytes[position]);
    //         }
    //     }
    //     return dp[position].unwrap();
    // }
    // pub fn push_dominoes(dominoes: String) -> String {
    //     let dp = vec![None: Option<u8>; dominoes.len()];
    //     let str_bytes = dominoes.as_bytes();
    //     let mut result: Vec<u8> = vec![];
    //     for index in 0..str_bytes.len() {
    //         result.push(Self::get_force_direction(index, &str_bytes, &dp));
    //     }
    //     match String::from_utf8(result) {
    //         Ok(v) => return v,
    //         Err(err) => panic!("Invalid bytes sequence for string: {}", err)
    //     };
    // }

    pub fn push_dominoes(mut dominoes: String) -> String {
        let mut temp: String = String::from("");
        while dominoes.ne(&temp) {
            temp = dominoes.clone();
            dominoes = dominoes.replace("R.L", "xxx").replace("R.", "RR").replace(".L", "LL");
        }
        return dominoes.replace("xxx", "R.L");
    }
}

mod test {
    #[test]
    #[ignore]
    fn p0833() {
        use crate::Solution;
        assert!(String::from("RR.L").eq(&Solution::push_dominoes(String::from("RR.L"))));
        assert!(String::from("LL.RR.LLRRLL..").eq(&Solution::push_dominoes(String::from(".L.R...LR..L.."))));
    }
}

