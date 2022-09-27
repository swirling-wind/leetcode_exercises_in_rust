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
    fn p0022() {
        use crate::Solution;

        for num in 1..8 {
            let outputs = Solution::generate_parenthesis(num);
            for item in outputs {
                print!("{item} \t");
            }
            println!()
        }
    }
}
