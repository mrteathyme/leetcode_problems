pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::BTreeMap;
    let mut m = BTreeMap::new();
    for x in 0..nums.len() {
        match m.get(&(target-nums[x])) {
                Some(y) => {return vec![*y as i32,x as i32];},
                None => {m.insert(nums[x], x);}
        }
};
vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_sum_test() {
        let list = vec![1,3,5,7,9,11,13,15,17,19];
        let target = 28;
        let indices = two_sum(list.clone(),target);
        assert_eq!(list[indices[0] as usize]+list[indices[1] as usize], target);
    }
}