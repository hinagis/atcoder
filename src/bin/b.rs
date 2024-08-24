fn main() {
    proconio::input! {mut a: [u32]}
    let mut c = 0;
    loop {
        a.sort_by(|a, b| b.cmp(a));
        if a[1] == 0 {break}
        c += 1;
        a[0] -= 1;
        a[1] -= 1;
    }
    println!("{}", c);
}
