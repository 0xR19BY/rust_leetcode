/*
This overflows with large integers when the integer is reversed
*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let stringified= &x.to_string();
        let vectorfied: Vec<char> = stringified
            .chars()
            .collect();
        let flipped: Vec<char> = vectorfied
            .into_iter()
            .rev()
            .collect();
        let concatenated: i32 = flipped
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        if &concatenated == &x {
            return true;
        }else{
            return false;
        }
    }
}
/*
This compares the strings. It has okay complexity (O(Log(N)))
*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        s == s.chars().rev().collect::<String>()
    }
}
