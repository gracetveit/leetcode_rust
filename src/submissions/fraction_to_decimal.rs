use super::Solution;

use std::collections::HashMap;

impl Solution {
    /// Solution for https://leetcode.com/problems/fraction-to-recurring-decimal/
    pub fn fraction_to_decimal(n: i32, d: i32) -> String {
        println!("{:?}", n);
        if n == 0 {
            return String::from("0");
        }
        if n as i64 % d as i64 == 0 {
            return (n as f64 / d as f64).to_string();
        }
        let repeating_element = Solution::does_repeat((n as i64).abs(), (d as i64).abs());
        let whole_decimal_as_string = (n as f64 / d as f64).to_string();
        let whole_as_string = whole_decimal_as_string.split('.').collect::<Vec<&str>>()[0];

        let mut return_string: String = String::from("");

        return_string.push_str(whole_as_string);
        return_string.push('.');
        return_string.push_str(&repeating_element);

        return_string
    }

    fn does_repeat(n: i64, d: i64) -> String {
        let mut res = String::from("");

        let mut remainders: HashMap<i64, usize> = HashMap::new();
        let mut rem = n % d;
        println!("{:?}", rem);
        while (rem != 0) && (!remainders.contains_key(&rem)) {
            remainders.insert(rem, res.len());

            rem *= 10;

            res.push_str(&(rem as f64 / d as f64).floor().to_string());

            rem = rem % d;
        }
        if remainders.contains_key(&rem) {
            let repeating = String::from(&res[*remainders.get(&rem).unwrap()..]);
            Solution::replace_repeating_element(res, repeating)
        } else {
            println!("{:?}", res);
            res
        }
    }

    fn replace_repeating_element(decimal: String, repeating_element: String) -> String {
        let mut r_string: String = String::from("");
        for x in 0..decimal.len() {
            if decimal[x..x + repeating_element.len()] == repeating_element {
                r_string.push_str(&decimal[0..x]);
                r_string.push('(');
                r_string.push_str(&repeating_element);
                r_string.push(')');
                break;
            }
        }
        r_string
    }
}

#[test]
fn example_0() {
    assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5")
}

#[test]
fn example_1() {
    assert_eq!(Solution::fraction_to_decimal(2, 1), "2")
}

#[test]
fn example_2() {
    assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)")
}

#[test]
fn example_3() {
    assert_eq!(Solution::fraction_to_decimal(1, 6), "0.1(6)")
}

#[test]
fn example_4() {
    assert_eq!(Solution::fraction_to_decimal(0, -5), "0")
}

#[test]
fn example_5() {
    assert_eq!(Solution::fraction_to_decimal(7, -12), "-0.58(3)")
}

#[test]
fn example_6() {
    assert_eq!(Solution::fraction_to_decimal(1, 214748364), "0.00(000000465661289042462740251655654056577585848337359161441621040707904997124914069194026549138227660723878669455195477065427143370461252966751355553982241280310754777158628319049732085502639731402098131932683780538602845887105337854867197032523144157689601770377165713821223802198558308923834223016478952081795603341592860749337303449725)")
}

#[test]
fn example_7() {
    assert_eq!(
        Solution::fraction_to_decimal(-1, -2147483648),
        "0.0000000004656612873077392578125"
    )
}
