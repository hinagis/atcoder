fn main() {
    proconio::input! {s: String}
    let mut r = 0;
    let mut o = true;
    for c in s.chars() {
        if o {
            if c == 'o' {
                r += 1;
            } else {
                o = false;
            }
        } else {
            if c == 'i' {
                r += 1;
            } else {
                o = true;
            }
        }
    }
    println!("{}", r + if (s.len() + r) % 2 == 0 {0} else {1});
}
