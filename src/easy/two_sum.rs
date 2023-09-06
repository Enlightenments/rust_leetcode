 /**
 Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 You may assume that each input would have exactly one solution, and you may not use the same element twice.
 You can return the answer in any order.

 ```
 Example 1:
 Input: nums = [2,7,11,15], target = 9
 Output: [0,1]
 Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 Example 2:

 Input: nums = [3,2,4], target = 6
 Output: [1,2]
 Example 3:

 Input: nums = [3,3], target = 6
 Output: [0,1]

 Constraints:

 2 <= nums.length <= 104
 -109 <= nums[i] <= 109
 -109 <= target <= 109
 Only one valid answer exists.
```
 */

pub fn main(nums: Vec<i32>, target: i32) -> Vec<i32> {
     let nums1 = nums.clone();
     let mut res = vec![];
    for (i,n) in nums.iter().enumerate() {
        for (i1,n1) in nums1.iter().enumerate() {
            if i!=i1 && n+n1==target {
                let num1 = i as i32;
                let num2 = i1 as i32;
                res = vec![num2,num1];
            }
        }
    }
    res
}
