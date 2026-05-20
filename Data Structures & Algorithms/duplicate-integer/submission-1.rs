use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hset = HashSet::<&i32>::with_capacity(nums.len());
        let res = nums.iter().try_for_each(|n: &i32| {
                if hset.insert(n) {
                    Ok(())
                } else {
                    Err(())
                }
            }
        );
        res.is_err()
    }
}
