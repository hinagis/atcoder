use proconio::{input as I, marker::{Usize1 as U, Chars as C}};

fn main() {
    I! {
        h: usize,
        w: usize,
        mut x: U,
        mut y: U,
        s: [C; h],
        t: C
    }
    let mut c = 0;
    let mut f = vec![vec![true; w]; h];
    for m in t {
        match m {
            'U' => {if s[x - 1][y] != '#' {x -= 1}}
            'D' => {if s[x + 1][y] != '#' {x += 1}}
            'L' => {if s[x][y - 1] != '#' {y -= 1}}
            'R' => {if s[x][y + 1] != '#' {y += 1}}
            _ => ()
        }
        if s[x][y] == '@' && f[x][y] {
            c += 1;
        }
        f[x][y] = false;
    }
 
    println!("{} {} {c}", x + 1, y + 1);
}
