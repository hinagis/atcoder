use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let mut v = (0, 0, 0u64);
    for &c in &s {
        match c {
            'o' => v.0 += 1,
            'x' => v.1 += 1,
            '?' => v.2 += 1,
            _ => (),
        }
    }

    if v.1 >= 10 {
        println!("0")
    } else {
        println!("{}",
            match v.0 {
                0 => v.2.pow(4),
                1 => 1  +     4 * v.2 +     6 * v.2.pow(2) + 4 * v.2.pow(3),
                2 => 14 + 6 * 4 * v.2 + 2 * 6 * v.2.pow(2),
                3 => 6 * 2 * 3 + 6 * 4 * v.2,
                4 => 24,
                _ => 0,
            }
        )
    }
}
