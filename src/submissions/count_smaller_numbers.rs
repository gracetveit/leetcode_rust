use super::Solution;

impl Solution {
    /// Solution for https://leetcode.com/problems/count-of-smaller-numbers-after-self/submissions/
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return vec![0];
        }
        let mut return_vec: Vec<i32> = vec![];
        for x in 0..nums.len() {
            return_vec.push(Solution::count_smaller_than_x(nums[x], &nums[x..]));
        }

        return return_vec;
    }

    fn count_smaller_than_x(x: i32, nums: &[i32]) -> i32 {
        let mut count: i32 = 0;
        for y in nums {
            if &x > y {
                count += 1
            }
        }
        count
    }
}

#[test]
fn example_0() {
    assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), [2, 1, 1, 0])
}

#[test]
fn example_1() {
    assert_eq!(Solution::count_smaller(vec![-1]), [0])
}

#[test]
fn example_2() {
    assert_eq!(Solution::count_smaller(vec![-1, -1]), [0, 0])
}
