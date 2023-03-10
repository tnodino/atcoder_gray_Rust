// https://atcoder.jp/contests/abc041/tasks/abc041_b

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    println!("{}", A * B % MOD * C % MOD);
}