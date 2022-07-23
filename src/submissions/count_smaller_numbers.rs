use super::Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return vec![0]
        }
        let mut return_vec: Vec<i32> = vec![];
        for x in 0..nums.len() {
            // for y in &nums[x..] {
            //     if &nums[x] > y {
            //         count += 1;
            //     }
            // }
            // return_vec.push(count);
            let count = &nums[x + 1..].into_iter().filter(|y| nums[x] > **y).collect::<Vec<&i32>>().len();
            return_vec.push(*count as i32);
        }

        return return_vec
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
