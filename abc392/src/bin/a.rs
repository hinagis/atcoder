fn main() {
    proconio::input! {a: [u32; 3]}
    println!("{}", if a[0] * a[1] == a[2] || a[1] * a[2] == a[0] || a[2] * a[0] == a[1] { "Yes" } else { "No" });
}
