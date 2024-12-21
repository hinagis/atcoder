fn main() {
    proconio::input! {
        n: usize,
        d: [u32; n]
    }
    let mut c = 0;
    for i in 0..n {
        let m = (i as u32 + 1) % 10;
        if m == 0 || (i > 9 && (i as u32 + 1) / 10 != m) {continue}
        if m <= d[i] {c += 1}
        if m * 10 + m <= d[i] {c += 1}
    }
    println!("{}", c);
}
