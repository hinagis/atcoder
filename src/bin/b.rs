fn main() {
    for i in 0..8 {
        proconio::input! {s: String}
        if s == "........" {continue}
        for (j, c) in s.chars().enumerate() {
            if c == '.' {continue}
            println!("{}{}", (j as u8 + b'a') as char, 8 - i);
            return;
        }
    }
}
