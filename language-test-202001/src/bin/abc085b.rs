fn rl() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let mut h: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let n = rl().trim().parse().unwrap();
    for _ in 0..n {
        h.insert(rl().trim().parse().unwrap());
    }
    println!("{}", h.len());
}
