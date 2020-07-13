#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count_one = 0;
    let mut count_four = 0;
    for i in 0..n {
        if a[i] % 4 == 0 {
            count_four += 1;
        } else if a[i] % 2 == 0 {
        } else {
            count_one += 1;
        }
    }
    if count_four >= n / 2 {
        println!("Yes");
    } else if count_one > count_four {
        println!("No");
    } else {
        println!("Yes");
    }
}
