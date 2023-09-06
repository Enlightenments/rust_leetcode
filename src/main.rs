mod easy;
use crate::easy::*;

fn main(){
    println!("hello");
    //
    let res = two_sum::main(vec![2,7,11,15],9);
    println!("{:?}",res);
    let res = palindrome_number::main(121);
    println!("{:?}",res);
    //
}
