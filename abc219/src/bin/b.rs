use proconio::{input, fastout, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
        t: Bytes,
    }
    for &t in &t {
        print!("{}", s[(t - b'0' - 1) as usize]);
    }
    println!("");
}
