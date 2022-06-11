use proconio::{input as I, marker::Usize1 as U};
fn main() {
    I! {
        r: U,
        c: U,
        a: [[String; 2]; 2]
    }
    println!("{}", a[r][c]);
}
