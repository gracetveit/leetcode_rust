use super::Solution;

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
    }
}

#[test]
fn list_node_conversion() {
    let test_data = vec![0, 1, 2, 3, 4];
    let test_list = ListNode::from_vec(test_data);

    assert_eq!(test_list.to_vec(), [0, 1, 2, 3, 4])
}
