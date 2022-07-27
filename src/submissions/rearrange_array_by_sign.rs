use crate::Solution;

impl Solution {
    /// Solution for
    /// [2149. Rearrange Array Elements by Sign](
    /// https://leetcode.com/problems/rearrange-array-elements-by-sign/
    /// )
    ///
    /// # Examples
    /// ```rust
    /// use leetcode_rust::Solution;
    ///
    /// assert_eq!(Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]), [3, -2, 1, -5, 2, -4])
    /// ```
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        // Split nums into positive & negative arrays
        let (positive, negative) = Solution::split_nums(nums);
        // return combined array
        Solution::combine_nums(positive, negative)
    }

    fn split_nums(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        nums
            .into_iter()
            .partition(|n| {
                n >= &0
            })
    }

    fn combine_nums(positive: Vec<i32>, negative: Vec<i32>) -> Vec<i32> {
        positive.into_iter()
            .zip(negative.into_iter())
            .map(|e| {
                [e.0, e.1]
            })
            .flatten()
            .collect()
    }
}
