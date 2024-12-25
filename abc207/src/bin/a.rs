fn main() {
    proconio::input! {mut a: [u32; 3]}
    a.sort();
    println!("{}", a[1] + a[2]);
}
