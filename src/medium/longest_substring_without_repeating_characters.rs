use std::collections::{HashMap};
use std::cmp::max;
/**
Given a string s, find the length of the longest substring without repeating characters.
```
Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.


Constraints:

0 <= s.length <= 5 * 104
s consists of English letters, digits, symbols and spaces.

solution：

LR pointer

L R
a b c a b c b b

L     R
a b c a b c b b

  L   R
a b c a b c b b

  L     R
a b c a b c b b

    L   R
a b c a b c b b

    L     R
a b c a b c b b
```
*/
pub fn main(s: String) -> i32 {
    let mut map = HashMap::new();
    let mut res = 0;
    let len = s.len();
    let mut start = 0;
    let charts:Vec<char> = s.chars().collect();
    for (i,&v) in charts.iter().enumerate() {
        if let Some(&index) = map.get(&v) {
            start = max(start,index+1);
        }
        res = max(res,i - start + 1);
        map.insert(v,i);
    }
    res as i32
}
