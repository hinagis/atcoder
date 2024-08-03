fn main() {
    proconio::input! {a: [u32]}
    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    a.sort_by(|a, b| b.1.cmp(a.1));
    println!("{}", a[1].0 + 1);
}
