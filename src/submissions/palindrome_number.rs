use crate::Solution;

impl Solution {
    /// Solution for https://leetcode.com/problems/palindrome-number/
    pub fn is_palindrome(x: i32) -> bool {
        println!(
            "{:?}, {:?}",
            x.to_string(),
            Solution::reverse_str(&x.to_string())
        );
        x.to_string() == Solution::reverse_str(&x.to_string())
    }

    fn reverse_str(input_string: &str) -> String {
        let mut return_string = String::from("");

        let string_chars: Vec<char> = input_string.chars().collect();

        let mut x: i32 = input_string.len() as i32 - 1;
        while x >= 0 {
            return_string.push(string_chars[x as usize]);
            x = x - 1;
        }

        return_string
    }
}

#[test]
fn example0() {
    assert_eq!(Solution::is_palindrome(121), true);
}

#[test]
fn example1() {
    assert_eq!(Solution::is_palindrome(-121), false);
}

#[test]
fn example2() {
    assert_eq!(Solution::is_palindrome(10), false);
}
