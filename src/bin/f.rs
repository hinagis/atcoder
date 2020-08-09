//use proconio::{input, fastout};
use proconio::input;

//#[fastout]
fn main() {
    input! {
        n: i32,
        d: [(i32, i32); n],
    }
    let r = n;
    for (_a, _b) in d {
        todo!();
    }

    println!("Yes");
    println!("{}", r);
}
