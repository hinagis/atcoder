fn main() {
    proconio::input! {
        _: usize,
        s: String
    }

    let mut t = vec![];
    let mut p = 0;
    for c in s.chars() {
        if p > 0 && c == ')' {
            while let Some(c) = t.pop() {
                if c == '(' {break}
            }
            p -= 1
        } else {
            if c == '(' {
                p += 1
            }
            t.push(c);
        }
    }
    println!("{}", t.iter().collect::<String>())
}
