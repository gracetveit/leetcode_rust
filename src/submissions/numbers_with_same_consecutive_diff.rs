use crate::Solution;

// Link: https://leetcode.com/problems/numbers-with-same-consecutive-differences/

impl Solution {
    /// Returns an arary of all integers of length `n` where the difference
    /// between every two consecutive digits is `k`.
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        //
        todo!()
    }
}

#[test]
fn example_0() {
    let ans = vec![181, 292, 707, 818, 929];
    assert_eq!(Solution::nums_same_consec_diff(3, 7), ans)
}

#[test]
fn example_1() {
    let ans = vec![
        10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
    ];
    assert_eq!(Solution::nums_same_consec_diff(2, 1), ans)
}
