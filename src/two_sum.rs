use std::collections::HashMap;

/// 输入：`nums = [2,7,11,15], target = 9`  
/// 输出：`[0,1]`  
/// 解释：`因为 nums[0] + nums[1] == 9 ，返回 [0, 1]。`  
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h_map: HashMap<&i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&pos) = h_map.get(&(target - num)) {
            return vec![pos as i32, i as i32];
        }
        h_map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;
    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(two_sum(nums, 3), vec![0, 1]);
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(two_sum(nums, 9), vec![3, 4]);
    }
}
