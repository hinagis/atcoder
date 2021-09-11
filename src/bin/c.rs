use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }
    let (tu, tl, tb, tr) = (get_u(&t, n), get_l(&t, n), get_b(&t, n), get_r(&t, n));
    fn chk(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>, n: usize, tu: usize, tl: usize, tb: usize, tr: usize) -> bool {
        let (su, sl, sb, sr) = (get_u(&s, n), get_l(&s, n), get_b(&s, n), get_r(&s, n));
        let v = sb - su;
        let h = sr - sl;
        if v == tb - tu && h == tr - tl {
            for i in 0..v {
                for j in 0..h {
                    if s[su + i][sl + j] != t[tu + i][tl + j] {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    };
    if chk(&s, &t, n, tu, tl, tb, tr) {
        println!("Yes");
        return;
    }

    let mut v = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            v[j][n - 1 - i] = s[i][j]
        }
    }
    if chk(&v, &t, n, tu, tl, tb, tr) {
        println!("Yes");
        return;
    }

    let mut v = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            v[n - 1 - i][n - 1 - j] = s[i][j]
        }
    }
    if chk(&v, &t, n, tu, tl, tb, tr) {
        println!("Yes");
        return;
    }

    let mut v = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            v[n - 1 - j][i] = s[i][j]
        }
    }
    if chk(&v, &t, n, tu, tl, tb, tr) {
        println!("Yes");
        return;
    }

    println!("No");
}

fn get_u(v: &Vec<Vec<char>>, n: usize) -> usize {
    for i in 0..n {
        for j in 0..n {
            if v[i][j] == '#' {
                return i;
            }
        }
    }
    n
}

fn get_l(v: &Vec<Vec<char>>, n: usize) -> usize {
    for j in 0..n {
        for i in 0..n {
            if v[i][j] == '#' {
                return j;
            }
        }
    }
    n
}

fn get_b(v: &Vec<Vec<char>>, n: usize) -> usize {
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if v[i][j] == '#' {
                return i + 1;
            }
        }
    }
    0
}

fn get_r(v: &Vec<Vec<char>>, n: usize) -> usize {
    for j in (0..n).rev() {
        for i in (0..n).rev() {
            if v[i][j] == '#' {
                return j + 1;
            }
        }
    }
    0
}
