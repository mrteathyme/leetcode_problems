pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
{ use std::collections::BTreeMap;

  let mut m = BTreeMap::new();

  for x in 0..nums.len() 
  { match m.get(&(target-nums[x])) 
    { Some(y) => {return vec![*y as i32,x as i32];},
      None => {m.insert(nums[x], x);} } };

 vec![] }

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> 
    { use std::collections::HashSet;

      if nums.len() == 3 
      { if nums[0] + nums[1] + nums[2] == 0 {return vec![nums]} }

      nums.sort_unstable();

      let mut results = HashSet::new();
      let mut previous_int = None;

      for x in 0..nums.len()-2 
      { if let Some(num) = previous_int 
        { if num == nums[x] 
          { continue }
          else 
          { previous_int = Some(nums[x]) } }
          else 
          { previous_int = Some(nums[x]) }

        let mut y = x + 1;
        let mut z = nums.len() - 1;
            
        while y < z 
        { let a = nums[y]+nums[z];

          if a < nums[x]*-1 
          { y+=1; } 
          else if a > nums[x]*-1 
          { z-=1; }
          else 
          { results.insert(vec![nums[x],nums[y],nums[z]]);
            y+=1;
            z-=1; } } }

     results.into_iter().collect() }

    #[cfg(test)]
mod tests 
{ use super::*;
 #[test]
 fn two_sum_test() 
 { let list = vec![1,3,5,7,9,11,13,15,17,19];
   let target = 28;
   let indices = two_sum(list.clone(),target);

   assert_eq!(list[indices[0] as usize]+list[indices[1] as usize], target); }

 #[test]
 fn three_sum_test() 
 { let test_case = vec![-1,0,1,2,-1,-4];
   let mut expected = vec![vec![-1,-1,2],vec![-1,0,1]];
   let mut result = three_sum(test_case);

   expected.sort();
   result.sort();

   assert_eq!(expected, result); } }