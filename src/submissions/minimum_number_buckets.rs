use crate::Solution;

/// # Example
/// ```rust
/// use leetcode_rust::Solution;
///
/// let street = String::from("H..H");
/// let answer = Solution::minimum_buckets(street);
/// assert_eq!(answer, 2);
/// ```
impl Solution {
    pub fn minimum_buckets(street: String) -> i32 {
        // Identify unreachable houses
        if Solution::unreachable_houses(&street) {
            return -1
        }
        // Identify Shared Buckets
        // 
        //
        // Identify Buckets attached to a house

        Solution::shared_buckets(&street)
    }

    fn unreachable_houses(street: &str) -> bool {
        let mut bucket_flag = false;
        let mut needs_bucket = false;
        for x in street.chars() {
            if x == '.' {
                if needs_bucket {
                    needs_bucket = false;
                    bucket_flag = true;
                } else {
                    bucket_flag = true;
                }
            } else {
                if bucket_flag {
                    bucket_flag = false;
                    needs_bucket = false;
                } else if !needs_bucket{
                    needs_bucket = true;
                } else {
                    return true
                }
            }
        }
        if needs_bucket {
            return true
        } else {
            return false
        }
    }

    fn shared_buckets(street: &str) -> i32 {
        let mut count = street.matches("H.H").collect::<Vec<&str>>().len();
        count += street.split("H.H")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x|{String::from(x)})
            .reduce(|acc: String, cur: String| -> String{
                format!("{}{}", acc, cur)
            })
            .unwrap()
            .matches("H")
            .collect::<Vec<&str>>()
            .len();
        count as i32
    }

    fn attached_to_house((remaining_houses, count): (String, i32)) -> i32 {
        todo!();
    }



}

#[test]
fn middle_check(){
    assert_eq!(Solution::minimum_buckets(String::from(".H.H.")), 1);
}

#[test]
fn impossible_check(){
    assert_eq!(Solution::minimum_buckets(String::from(".HHH.")), -1);
}

#[test]
fn example_0(){
    assert_eq!(Solution::minimum_buckets(String::from(".HH.HH.HH.HH..H")), 6)
}
