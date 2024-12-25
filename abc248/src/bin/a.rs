use proconio::{input as I, marker::Bytes};
fn main() {
    I! {s: Bytes}
    for i in b'0'..=b'9' {
        if ! s.contains(&i) {
            println!("{}", i as char);
            return;
        }
    }
}
