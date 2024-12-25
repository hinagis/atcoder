fn main() {
    proconio::input! {
        s: String,
        t: String,
    }
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let mut r = 0;
    for i in 0..s.len() {
        if s[i] != t[i] {
            r += 1;
        }
    }

    println!("{}", r);
}
