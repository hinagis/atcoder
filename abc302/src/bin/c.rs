use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        m: usize,
        s: [C; n]
    }
    fn jdg(s: &Vec<Vec<char>>, n: usize, m: usize, b: u32, p: usize, f: u32) -> bool {
        if f == b {
            return true;
        }
        for i in 0..n {
            if f & 1 << i == 0 {
                let mut c = 0;
                for j in 0..m {
                    if s[p][j] != s[i][j] {
                        c += 1;
                    }
                }
                if c <= 1 && jdg(s, n, m, b, i, f | 1 << i) {
                    return true;
                }
            }
        }
        return false;
    }
    let b = match n {
        1 => 0x1,
        2 => 0x3,
        3 => 0x7,
        4 => 0xf,
        5 => 0x1f,
        6 => 0x3f,
        7 => 0x7f,
        8 => 0xff,
        _ => 0
    };
    for i in 0..n {
        if jdg(&s, n, m, b, i, 1 << i) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
