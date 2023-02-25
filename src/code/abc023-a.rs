// https://atcoder.jp/contests/abc023/tasks/abc023_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", X / 10 + X % 10);
}