fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        a: [i32; n]
    }
    let h: std::collections::HashSet<&i32> = a.iter().collect();
    for &y in &h {
        if h.contains(&(x + y)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
