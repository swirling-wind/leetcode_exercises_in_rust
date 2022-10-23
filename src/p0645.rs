use crate::Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut num_remains: Vec<i8> = vec![1; nums.len() + 1];
        for n in nums {
            num_remains[n as usize] -= 1;
        }
        let mut result: Vec<i32> = vec![0,0];
        for index in 0..num_remains.len() {
            if num_remains[index] < 0 {
                result[0] = index as i32;
            }
            else if num_remains[index] > 0 {
                result[1] = index as i32;
            }
        }
        return result;
    }
}

mod test {
    #[test]
    #[ignore]
    fn p0658() {
        use crate::Solution;
        dbg!(Solution::find_error_nums(vec![1,2,2,4]));
        dbg!(Solution::find_error_nums(vec![4,2,3,4]));
        dbg!(Solution::find_error_nums(vec![1,2,3,4,4]));
        dbg!(Solution::find_error_nums(vec![3,2,2]));
    }
}
