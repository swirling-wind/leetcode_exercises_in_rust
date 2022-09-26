use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count: [i32; 128] = [0; 128];
        for (_, ch) in s.as_bytes().iter().enumerate() {
            count[*ch as usize] += 1;
        }
        let mut ans = 0;
        for value in count {
            ans += value / 2 * 2;
            if ans % 2 == 0 && value % 2 == 1 {
                ans += 1;
            }
        }
        ans
    }
}

mod test {
    use crate::Solution;

    //noinspection SpellCheckingInspection
    #[test]
    fn p0001() {
        assert_eq!(7, Solution::longest_palindrome(String::from("abccccdd")));
        assert_eq!(1, Solution::longest_palindrome(String::from("a")));
    }
}