mod easy;
mod medium;

use crate::easy::*;
use crate::medium::*;

fn main(){
    println!("hello");
    //
    let res = two_sum::main(vec![2,7,11,15],9);
    println!("{:?}",res);
    let res = palindrome_number::main(121);
    println!("{:?}",res);
    let res = longest_substring_without_repeating_characters::main("abcabcbb".to_string());
    println!("{:?}",res);
    //
}
