use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}

    for i in (0..s.len()).rev() {
        print!("{}", match s[i] {
            '6' => '9',
            '9' => '6',
            c => c,
        });
    }
    println!("");
}
