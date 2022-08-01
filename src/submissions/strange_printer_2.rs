// [1 2 1]
// [2 1 2]
// [1 2 1]

use crate::Solution;

impl Solution {
    /// Solving for [LeetCode #1591: Strange Printer II](https://leetcode.com/problems/strange-printer-ii/)
    ///
    /// # Examples
    /// ```rust
    /// use leetcode_rust::Solution;
    ///
    /// let answer = Solution::is_printable(
    ///     vec![
    ///         vec![1, 1, 1, 1],
    ///         vec![1, 1, 3, 3],
    ///         vec![1, 1, 3, 4],
    ///         vec![5, 5, 1, 4]
    ///     ]
    /// );
    /// assert_eq!(answer, true);
    /// ```
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        // Construct a TwoDimGrid object
        // For each number, make sure that each element => num is congruous and
        // rectangular(???)
        // Increment num, and repeat until all elements are the same num
        todo!()
    }
}

struct TwoDimGrid {
    grid: Result<Vec<Vec<i32>>, &'static str>
}

impl TwoDimGrid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        TwoDimGrid {
            grid: Ok(grid)
        }
    }

    // TODO: Figure out how monad functions would return self using pattern
    // matching
    fn smallest_is_rectangular(&self) -> Self {
        match self.grid {
            Ok(x) => {
                todo!()
            }
            err => return {
                TwoDimGrid {
                    grid: self.grid
                }
            }
        }
    }
}
