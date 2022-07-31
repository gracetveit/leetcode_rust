use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Converts a ListNode into a vec
    pub fn to_vec(&self) -> Vec<i32> {
        match &self.next {
            Some(x) => {
                return vec![vec![self.val], x.to_vec()]
                    .into_iter()
                    .flatten()
                    .collect()
            }
            None => return vec![self.val],
        }
    }

    /// Converts a vec into a ListNode
    ///
    /// # Panics
    /// ```rust,should_panic
    /// use leetcode_rust::submissions::add_two_numbers::ListNode;
    ///
    /// // panics if data has no values
    /// ListNode::from_vec(vec![]);
    /// ```
    pub fn from_vec(data: Vec<i32>) -> Self {
        match data.len() {
            0 => panic!(),
            1 => ListNode {
                val: data[0],
                next: None,
            },
            _ => ListNode {
                val: data[0],
                next: Some(Box::new(ListNode::from_vec(Vec::from(&data[1..])))),
            },
        }
    }
}
impl Solution {
    /// Solution for [LeetCode Problem #2, Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)
    ///
    /// # Examples
    /// ```rust
    /// use leetcode_rust::submissions::add_two_numbers::ListNode;
    /// use leetcode_rust::Solution;
    ///
    /// let l1 = ListNode::from_vec(vec![2, 4, 3]);
    /// let l2 = ListNode::from_vec(vec![5, 6, 4]);
    ///
    /// let sum = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    ///
    /// match sum {
    ///     Some(x) => {
    ///         assert_eq!(x.to_vec(), [7, 0, 8])
    ///     },
    ///     None => panic!()
    /// }
    /// ```
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (_1, _2) => Solution::recurring_sum(0, _1, _2),
        }
    }

    fn recurring_sum(
        cur_sum: i32,
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (cur_sum, l1, l2) {
            (0, None, None) => None,
            (_sum, _l1, _l2) => {
                let new_sum = Solution::add_with_rollover(_sum, Solution::option_sum(&_l1, &_l2));
                let next_nodes = Solution::unpack_list_node(_l1, _l2);
                let mut new_node = ListNode::new(new_sum.0);
                let next_node = Solution::recurring_sum(new_sum.1, next_nodes.0, next_nodes.1);
                new_node.next = next_node;
                return Some(Box::new(new_node));
            }
        }
    }

    fn unpack_list_node(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        match (l1, l2) {
            (None, None) => (None, None),
            (Some(x), None) => (x.next, None),
            (None, Some(x)) => (x.next, None),
            (Some(x), Some(y)) => (x.next, y.next),
        }
    }

    fn option_sum(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> i32 {
        match (l1, l2) {
            (None, None) => 0,
            (Some(x), Some(y)) => x.val + y.val,
            (Some(x), None) => x.val,
            (None, Some(x)) => x.val,
        }
    }

    fn add_with_rollover(a: i32, b: i32) -> (i32, i32) {
        let sum = a + b;
        if sum >= 10 {
            return (sum % 10, (sum as f64 / 10.0).floor() as i32);
        } else {
            return (sum, 0);
        }
    }
}

#[test]
fn list_node_conversion() {
    let test_data = vec![0, 1, 2, 3, 4];
    let test_list = ListNode::from_vec(test_data);

    assert_eq!(test_list.to_vec(), [0, 1, 2, 3, 4])
}
