fn main() {
    proconio::input! {s: String}
    let mut r = 0u32;
    let mut f = false;
    for c in s.chars() {
        r += if f {1} else {0};
        if c == '0' {
            f ^= true;
        } else {
            r += 1;
            f = false;
        }
    }
    println!("{}", r + if f {1} else {0});
}
