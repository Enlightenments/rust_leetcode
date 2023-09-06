/**
```
Given an integer x, return true if x is a
palindrome , and false otherwise.

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


Constraints:

-231 <= x <= 231 - 1

```
*/

pub fn main(x: i32) -> bool {
    if x <0 {
        false
    }else if x==0 {
        true
    }else{
        let x_str = x.to_string();
        let mut rev_str = String::new();
        for i in x_str.chars().rev() {
            rev_str.push(i);
        }
        x_str == rev_str
    }
}
