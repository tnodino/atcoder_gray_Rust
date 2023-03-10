// https://atcoder.jp/contests/arc031/tasks/arc031_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Name: String,
    }
    let vec = Name.chars().collect::<Vec<char>>();
    let mut rev_vec = vec.clone();
    rev_vec.reverse();
    if vec == rev_vec {
        println!("YES");
    }
    else {
        println!("NO");
    }
}