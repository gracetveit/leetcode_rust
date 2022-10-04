use crate::Solution;

// Link: https://leetcode.com/problems/numbers-with-same-consecutive-differences/

impl Solution {
    /// Returns an arary of all integers of length `n` where the difference
    /// between every two consecutive digits is `k`.
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let n_digit_integers = Solution::generate_n_digit_integers(n);
        n_digit_integers
            .into_iter()
            .filter(|x| Solution::consecutive_digits_is_k_long(x, k))
            .collect()
    }

    fn generate_n_digit_integers(n: i32) -> Vec<i32> {
        let mut digits = Vec::new();
        for _ in 0..n {
            digits.push(String::from("0"));
        }
        digits[0] = String::from("1");
        let floor = digits.join("").parse::<i32>();

        for i in 0..digits.len() {
            digits[i] = String::from("9");
        }
        let top = digits.join("").parse::<i32>();

        match (floor, top) {
            (Ok(floor), Ok(top)) => {
                let mut int_vec = Vec::new();
                for i in floor..top + 1 {
                    int_vec.push(i)
                }
                int_vec
            }
            (_, _) => panic!(),
        }

        // todo!()
    }

    fn consecutive_digits_is_k_long(n: &i32, k: i32) -> bool {
        let digits_as_string = n.to_string();

        let digits = digits_as_string
            .split("")
            .filter(|e| e != &"")
            .map(|e| {
                let res_e = e.parse::<i32>();
                match res_e {
                    Ok(res_e) => res_e,
                    _ => {
                        panic!()
                    }
                }
            })
            .collect::<Vec<i32>>();

        for x in 1..digits.len() {
            let y = x - 1;

            let diff = digits[x] - digits[y];
            if diff.abs() != k {
                return false;
            }
        }
        true
        // println!("{:?}", digits);
        // todo!()
    }
}

#[test]
fn example_0() {
    let ans = vec![181, 292, 707, 818, 929];
    assert_eq!(Solution::nums_same_consec_diff(3, 7), ans);
}

#[test]
fn example_1() {
    let ans = vec![
        10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
    ];
    assert_eq!(Solution::nums_same_consec_diff(2, 1), ans);
}

#[test]
fn test_generate_n_digit_integers() {
    let ans = Solution::generate_n_digit_integers(2);
    assert_eq!(ans[0], 10);
    assert_eq!(ans[ans.len() - 1], 99);
}

#[test]
fn test_consecutive_digits_is_k_long() {
    assert_eq!(Solution::consecutive_digits_is_k_long(&181, 7), true);
    assert_eq!(Solution::consecutive_digits_is_k_long(&101, 7), false);
    assert_eq!(Solution::consecutive_digits_is_k_long(&10, 1), true);
    assert_eq!(Solution::consecutive_digits_is_k_long(&13, 1), false);
}
