use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for (idx, &val) in nums.iter().enumerate() {
            if let Some(old_index) = map.insert(val, idx) {
                return true
            }
        }
        return false
    }
}
