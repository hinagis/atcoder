use proconio::{input as I, marker::Bytes as B};
fn main() {
    I! {s: B}
    let mut u = false;
    let mut l = false;
    let mut a = [false; 256];
    for &c in &s {
        u |= c <= b'Z';
        l |= c >= b'a';
        let c = c as usize;
        if a[c] {
            println!("No");
            return;
        }
        a[c] = true;
    }

    println!("{}", if u && l {"Yes"} else {"No"});
}
