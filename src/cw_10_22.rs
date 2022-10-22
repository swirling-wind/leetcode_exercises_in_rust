use std::vec;

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    result.reserve(arr.len());
    let mut count = 0;
    for &num in arr {
       if num == 0 {
            count += 1;
       } else {
        result.push(num);
       }
    }
    let mut tail: Vec<u8> = vec![0; count];
    result.append(&mut tail);
    result
}

fn sum_of_divided(input: Vec<i64>) -> Vec<(i64, i64)> {
    // let is_prime = |value: usize|->bool {
    //     for i in 2..=((value as f32).sqrt().floor() as usize) {
    //         if value % i == 0 {
    //             return false;
    //         }
    //     }
    //     return true;
    // };

    let is_prime = |value: usize|->bool {
        match value {
            2|3 => {
                return true;
            },
            x if x % 6 != 1 && x % 6 != 5 => {
                return false;
            }
            _ => {
                for num in (5..=(value as f32).sqrt().floor() as usize).step_by(6) {
                    if value % num == 0 || value % (num + 2) == 0 {
                        return false;
                    }
                }
                return true;
            }
        };
    };
    
    let mut result: Vec<(i64, i64)> = vec![];
    for index in 2..=((input.iter().map(|&x| x.abs()).max().unwrap_or(0)) as usize) {
        if is_prime(index) {
            if input.iter().any(|&x| x % index as i64 == 0) {
                result.push((index as i64, input.iter().filter(|&&x| x % index as i64 == 0).sum()));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(actual == expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")   
    }
    
    #[test]
    #[ignore]
    fn sample_tests() {
        dotest(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        dotest(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }

    fn test_sum_of_divided(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(sum_of_divided(l), exp)
    }
    
    #[test]
    fn basics_sum_of_divided() {
        test_sum_of_divided(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
        test_sum_of_divided(vec![12, 5], vec![ (2, 12), (3, 12), (5, 5) ]);
        test_sum_of_divided(vec![], vec![]);
        test_sum_of_divided(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);
        test_sum_of_divided(vec![15,30,-45], vec![ (2, 30), (3, 0), (5, 0) ]);
        test_sum_of_divided(vec![79], vec![ (79,79) ]);
    }
}

