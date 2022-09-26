use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut records: HashMap<i32, i32> = HashMap::new();
        for (index, &value) in nums.iter().enumerate() {
            if records.contains_key(&(target - value)) {
                return vec![index as i32, records[&(target - value)]];
            }
            records.insert(value, index as i32);
        }
        return vec![];
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn p0001() {
        assert!(Solution::two_sum([2, 7, 11, 15].to_vec(), 9).iter().all(|item| vec![1, 0].contains(item)));
        assert!(Solution::two_sum([3, 2, 4, 12].to_vec(), 6).iter().all(|item| vec![1, 2].contains(item)));
        assert!(Solution::two_sum([0, 3, 1, 3, 15].to_vec(), 6).iter().all(|item| vec![1, 3].contains(item)));
    }
}

