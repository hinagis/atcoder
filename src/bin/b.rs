//use proconio::{input, fastout};
use proconio::input;

//#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        _c: i64,
        k: i64,
    }
    if a >= k {
        println!("{}", k);
    } else if a + b >= k {
            println!("{}", a);
    } else {
        println!("{}", a - (k - a - b));
    }
}
