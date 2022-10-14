use crate::Solution;

impl Solution {
    fn add_parenthesis(result: &mut Vec<String>, current_str: String, counter: i32, left: i32, right: i32) {
        if counter >= 0 {
            if left == 0 && right == 0 {
                result.push(current_str);
            } else {
                // Add '('
                if left > 0 {
                    let mut next_str = current_str.clone();
                    next_str.push('(');
                    Self::add_parenthesis(result, next_str, counter + 1, left - 1, right);
                }
                // Add ')'
                if right > 0 {
                    let mut next_str = current_str.clone();
                    next_str.push(')');
                    Self::add_parenthesis(result, next_str, counter - 1, left, right - 1);
                }
            }
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        Self::add_parenthesis(&mut result, String::from(""), 0, n, n);
        return result;
    }
}

// impl AnotherSolution {
//     const fn save_add_parenthesis(result: &mut Vec<String>, current_str: String, counter: i32, left: i32, right: i32) {
//         if counter >= 0 {
//             if left == 0 && right == 0 {
//                 result.push(current_str);
//             } else {
//                 // Add '('
//                 if left > 0 {
//                     let mut next_str = current_str.clone();
//                     next_str.push('(');
//                     Self::save_add_parenthesis(result, next_str, counter + 1, left - 1, right);
//                 }
//                 // Add ')'
//                 if right > 0 {
//                     let mut next_str = current_str.clone();
//                     next_str.push(')');
//                     Self::save_add_parenthesis(result, next_str, counter - 1, left, right - 1);
//                 }
//             }
//         }
//     }
//
//     const fn save_parenthesis()-> Vec<Vec<String>> {
//         let mut all_results: Vec<Vec<String>> = vec![];
//         {
//             let mut result: Vec<String> = vec![];
//             Self::save_add_parenthesis(&mut result, String::from(""), 0, 1, 1);
//             all_results.push(result);
//         }
//         {
//             let mut result: Vec<String> = vec![];
//             Self::save_add_parenthesis(&mut result, String::from(""), 0, 2, 2);
//             all_results.push(result);
//         }
//         {
//             let mut result: Vec<String> = vec![];
//             Self::save_add_parenthesis(&mut result, String::from(""), 0, 3, 3);
//             all_results.push(result);
//         }
//         all_results
//     }
// }

mod test {
    #[test]
    #[ignore]
    fn p0022() {
        use crate::Solution;
        let expects = vec![
            vec![],
            vec![String::from("()")],
            vec![String::from("(())"), String::from("()()")],
            vec![String::from("((()))"), String::from("(()())"),
                 String::from("(())()"), String::from("()(())"), String::from("()()()")]];

        for num  in 1..4_usize {
            let outputs = Solution::generate_parenthesis(num as i32);
            for index in 0..outputs.len() {
                assert!(outputs[index].eq(&expects[num][index]));
            }
        }
    }
}
