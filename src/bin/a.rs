fn main() {
    const S: [&str; 5] = ["and", "not", "that", "the", "you"];
    proconio::input! {w: [String]}
    for w in &w {
        if S.contains(&w.as_str()) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
