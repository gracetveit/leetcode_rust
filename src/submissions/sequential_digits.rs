use crate::Solution;

// https://leetcode.com/problems/sequential-digits/

impl Solution {
    /// Returns a sorted list of all integers in the range `[low, high]`
    /// inclusive that have sequential digits
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        // Generate vector of integers, inclusive
        let mut int_vec: Vec<i32> = Vec::new();
        for i in low..high + 1 {
            if Solution::is_sequential(i) {
                int_vec.push(i)
            }
        }

        // filter integers to only include those with inclusive digits
        int_vec
    }

    fn is_sequential(n: i32) -> bool {
        let parsed_n: Vec<i32> = n
            .to_string()
            .split("")
            .filter(|e| e != &"")
            .map(|e| match e.parse::<i32>() {
                Ok(e) => e,
                Err(_) => panic!(),
            })
            .collect();

        for y in 1..parsed_n.len() {
            let x = y - 1;
            if parsed_n[y] != parsed_n[x] + 1 {
                return false;
            }
        }
        true
    }
}

#[test]
fn example_0() {
    assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234])
}

#[test]
fn example_1() {
    assert_eq!(
        Solution::sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    )
}

#[test]
fn test_is_sequential() {
    assert_eq!(Solution::is_sequential(1234), true);
    assert_eq!(Solution::is_sequential(8392), false);
}
