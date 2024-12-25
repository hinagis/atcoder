fn main() {
    proconio::input! {s: String}
    let mut a = Vec::with_capacity(30);
    a.push(0u32);
    for c in s.bytes() {
        if c == b'(' {
            a.push(*a.last().unwrap());
        } else if c == b')' {
            a.pop();
        } else {
            let b = 1 << c - b'a';
            if *a.last().unwrap() & b != 0 {
                println!("No");
                return;
            } else {
                *a.last_mut().unwrap() |= b;
            }
        }
    }
    println!("Yes");
}
