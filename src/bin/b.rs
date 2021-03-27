use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [Chars; h]
    }

    let mut c = 1;

    let mut i = x;
    let mut j = y + 1;
    while j <= w && s[i - 1][j - 1] != '#' {
        c += 1;
        j += 1;
    }
    j = y - 1;
    while j >= 1 && s[i - 1][j - 1] != '#' {
        c += 1;
        j -= 1;
    }
    j = y;
    i = x + 1;
    while i <= h && s[i - 1][j - 1] != '#' {
        c += 1;
        i += 1;
    }
    i = x - 1;
    while i >= 1 && s[i - 1][j - 1] != '#' {
        c += 1;
        i -= 1;
    }

    println!("{}", c);
}
