use super::Solution;

impl Solution {
    /// Solution for
    /// [2149. Rearrange Array Elements by Sign](
    /// https://leetcode.com/problems/rearrange-array-elements-by-sign/
    /// )
    ///
    /// # Examples
    /// ```rust
    /// use leetcode_rust::submissions::Solution;
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
        let mut positive: Vec<i32> = Vec::new();
        let mut negative: Vec<i32> = Vec::new();

        for x in nums {
            if x >= 0 {
                positive.push(x);
            } else {
                negative.push(x);
            }
        }

        (positive, negative)
    }

    fn combine_nums(positive: Vec<i32>, negative: Vec<i32>) -> Vec<i32> {
        let mut ret_vec: Vec<i32> = Vec::new();
        let mut p_index = 0;
        let mut n_index = 0;

        while p_index < positive.len() || n_index < negative.len() {
            if p_index < positive.len() {
                ret_vec.push(positive[p_index]);
                p_index += 1;
            }
            if n_index < negative.len() {
                ret_vec.push(negative[n_index]);
                n_index += 1;
            }
        }

        ret_vec
    }
}
