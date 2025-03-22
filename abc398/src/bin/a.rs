fn main() {
    proconio::input! {n: usize}
    let e = if n % 2 == 0 {2} else {1};
    let h = "-".repeat((n - e) / 2);
    println!("{h}{}{h}", "=".repeat(e));
}
