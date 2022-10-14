use std::collections::BinaryHeap;
use crate::Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        let mut previous_end: Option<i32> = None;

        for item in buildings {
            let (start, end, height) = (item[0], item[1], item[2]);

            match previous_end {
                Some(ending) =>
                    if ending < start {
                        let change_height = vec![];
                        result.push(change_height);
                    }
                ,
                None => {previous_end = Some(end);}
            };

            match heap.peek() {
                Some(current) => {
                    if height > *current {
                        let change_height = vec![start, height];
                        result.push(change_height);
                    }
                }
                None => {
                    let change_height = vec![start, height];
                    result.push(change_height);
                }
            };

            heap.push(height);
        }

        result
    }
}

mod test {
    #[test]
    #[ignore]
    fn p0218() {
        use crate::Solution;
        let input: Vec<Vec<i32>> = vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12],
                                        vec![15, 20, 10], vec![19, 24, 8]];
        let result = Solution::get_skyline(input);
        dbg!(result);
    }
}