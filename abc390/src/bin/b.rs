fn main() {
    proconio::input! {a: [u64]}
    for a in a.windows(3) {
        if a[1] * a[1] != a[0] * a[2] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
