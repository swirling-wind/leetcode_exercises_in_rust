use crate::Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, mut k: i32, x: i32) -> Vec<i32> {
        let mut low: i32 = 0;
        let mut high: i32 = (arr.len() - 1) as i32;
        let mut mid: i32 = 0;
        let mut find = false;

        while low < high {
            mid = (low + high) / 2;
            if x == arr[mid as usize] {
                find = true;
                break;
            } else if x > arr[mid as usize] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        let mut result: Vec<i32> = vec![];
        let start: i32 = if find { mid } else { low };

        println!("{}", start);

        let mut left: i32 = start - 1;
        let mut right: i32 = start;

        while k > 0 {
            if left < 0 || right >= arr.len() as i32 {
                if right >= arr.len() as i32 {
                    result.push(arr[left as usize]);
                    left -= 1;
                } else {
                    result.push(arr[right as usize]);
                    right += 1;
                }
            } else {
                if (x - arr[left as usize]).abs() <= (x - arr[right as usize]).abs() {
                    result.push(arr[left as usize]);
                    left -= 1;
                } else {
                    result.push(arr[right as usize]);
                    right += 1;
                }
            }
            k -= 1;
        }
        result.sort();
        result
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