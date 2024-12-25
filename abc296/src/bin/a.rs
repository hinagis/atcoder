fn main() {
    proconio::input! {
        _: usize,
        s: String
    }
    let mut p = ' ';
    for c in s.chars() {
        if c == p {
            println!("No");
            return;
        }
        p = c;
    }
    println!("Yes");
}
