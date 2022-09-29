use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end_recursive(head, n).0
    }

    fn remove_nth_from_end_recursive(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
        match head {
            None => (None, 1),
            Some(mut node) => {
                let (prev, num) = Self::remove_nth_from_end_recursive(node.next.take(), n);
                return if n == num as i32 {
                    (prev, num + 1)
                } else {
                    node.next = prev;
                    (Some(node), num + 1)
                }
            }
        }
    }
}


mod test {
    use crate::p0019::ListNode;
    use crate::Solution;

    #[test]
    fn p0019() {
        let input_0: Option<Box<ListNode>> = Some(Box::<ListNode>::new( ListNode::new(1)));
        let result_0 = Solution::remove_nth_from_end(input_0, 1);
        assert_eq!(result_0, None);
    }
}