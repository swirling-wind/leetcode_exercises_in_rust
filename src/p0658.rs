use crate::Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, mut k: i32, x: i32) -> Vec<i32> {
        let mut low = 0;
        let mut high = arr.len() as i32 - 1;
        while high - low >= k {
            if (arr[low as usize] - x).abs() > (arr[high as usize] - x).abs() {
                low += 1;
            } else {
                high -= 1;
            }
        }
        let mut result: Vec<i32> = vec![];
        for num in low..high+1 {
            result.push(arr[num as usize]);
        }
        return result;
    }
}

mod test {
    #[test]
    fn p0658() {
        use crate::Solution;

        let test_0 = vec![1, 5, 10];
        let output_0 = Solution::find_closest_elements(test_0, 1, 4);
        dbg!(output_0);
        //
        // let test_0 = vec![-1,1,2,4,5,9,12];
        // let output_0 = Solution::find_closest_elements(test_0, 4, 3);
        // dbg!(output_0);
        //
        // let test_0 = vec![1,3];
        // let output_0 = Solution::find_closest_elements(test_0, 1,2);
        // dbg!(output_0);
        //
        // let test_1 = vec![1, 2, 2, 3, 4, 5];
        // let output_1 = Solution::find_closest_elements(test_1, 4, -1);
        // dbg!(output_1);
        //
        // let test_1 = vec![1, 2, 3, 4, 4, 5];
        // let output_2 = Solution::find_closest_elements(test_1, 4, 9);
        // dbg!(output_2);
    }
}