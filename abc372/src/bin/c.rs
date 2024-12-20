use proconio::{input as I, fastout as F, marker::{Chars as C, Usize1 as U}};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        mut s: C
    }
    let mut p = s.iter().collect::<String>().match_indices("ABC").count();
    for _ in 0..q {
        I! {
            x: U,
            c: char
        }
        match s[x] {
            'A' => {
                if x + 2 < n && s[x + 1] == 'B' && s[x + 2] == 'C' {
                    p -= 1;
                }
            },
            'B' => {
                if x > 0 && x + 1 < n && s[x - 1] == 'A' && s[x + 1] == 'C' {
                    p -= 1;
                }
            },
            'C' => {
                if x > 1 && s[x - 2] == 'A' && s[x - 1] == 'B' {
                    p -= 1;
                }
            },
            _ => {}
        }
        match c {
            'A' => {
                if x + 2 < s.len() && s[x + 1] == 'B' && s[x + 2] == 'C' {
                    p += 1;
                }
            },
            'B' => {
                if x > 0 && x + 1 < s.len() && s[x - 1] == 'A' && s[x + 1] == 'C' {
                    p += 1;
                }
            },
            'C' => {
                if x > 1 && s[x - 2] == 'A' && s[x - 1] == 'B' {
                    p += 1;
                }
            },
            _ => {}
        }
        s[x] = c;
        println!("{}", p);
    }
}
