use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(value) = hashmap.get(num) {
            return vec![*value, i as i32];
        }
        let target_subtraction: i32 = target - num;
        hashmap.insert(target_subtraction, i as i32);
    }
    return nums;
}

pub fn main() {
    two_sum([2, 7, 11, 15].to_vec(), 9);
    two_sum([3, 2, 4].to_vec(), 6);
    two_sum([3, 3].to_vec(), 6);
}
