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
        TwoDimGrid::new(target_grid)
            .smallest_is_rectangular()
            .valid_print()
        // For each number, make sure that each element => num is congruous and
        // rectangular(???)
        // Increment num, and repeat until all elements are the same num
    }
}

#[derive(Clone)]
struct TwoDimGrid {
    grid: Result<Vec<Vec<i32>>, &'static str>,
}

impl TwoDimGrid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        TwoDimGrid { grid: Ok(grid) }
    }

    // Takes an (x, y) and increments each (x first) until it hits the bound of
    // the grid, *or* it encounters a smaller number.
    // Takes a slice of (0, 0) -> (x, y) (relative to starting pos) and checks
    // if there are any numbers lower than the given num
    //

    fn find_pos_of_int(&self, num: i32) -> (usize, usize) {
        todo!()
    }

    fn find_smallest_int(&self) -> i32 {todo!()}

    fn smallest_is_rectangular(&self) -> Self {
        match &self.grid {
            Ok(grid) => {
                let num = self.find_smallest_int();
                let (mut x, mut y) = self.find_pos_of_int(num);
                while x < grid[0].len() && y < grid.len() {

                    todo!()
                }
                todo!()
            }
            _err => self.clone(),
        }
    }

    fn valid_print(&self) -> bool {
        match &self.grid {
            Ok(_) => true,
            _err => false,
        }
    }
}
