 /* This is easiest tackled by converting the integer to a string, converting the characters into an
 * array, flipping the array, converting the array back to string -> back to integer and checking
 * if they equal
 *
 * THIS IS NOT YET COMPLETE.
 */

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let stringified= &x.to_string();
        let vectorfied: Vec<char> = stringified
            .chars()
            .collect::<Vec<char>>();
        let flipped: Vec<char> = vectorfied
            .reverse();
        let concatenated: i32 = &flipped
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

