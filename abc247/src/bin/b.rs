fn main() {
    proconio::input! {
        n: usize,
        s: [(String, String); n]
    }
    for i in 0..n {
        let mut f = false;
        for j in 0..n {
            if i != j && (s[i].0 == s[j].0 || s[i].0 == s[j].1) {
                f = true;
                break;
            }
        }
        if f {
            for j in 0..n {
                if i != j && (s[i].1 == s[j].0 || s[i].1 == s[j].1) {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
