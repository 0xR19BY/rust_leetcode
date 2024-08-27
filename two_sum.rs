use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>
    {
    let mut seen = HashMap::new();
    for (i, &num) in nums.iter().enumerate() 
    {
        let diff = target - num;
        if let Some(&index) = seen.get(&diff)
        {
            return vec![index as i32, i as i32];
        }
        seen.insert(num,i);
        }
    vec![]
    }
}
