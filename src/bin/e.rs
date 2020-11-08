use proconio::{input, marker::Usize1};

#[derive(Clone, std::cmp::PartialEq)]
enum B {
    L,
    W,
    N,
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
        cd: [(Usize1, Usize1); m],
    }
    let mut b = vec![vec![B::N; w]; h];
    for &(i, j) in &cd {
        b[i][j] = B::W;
    }
    for &(i, j) in &ab {
        b[i][j] = B::L;
        for i in (0..i).rev() {
            if b[i][j] == B::W {
                break;
            }
            b[i][j] = B::L;
        }
        for i in (i + 1)..h {
            if b[i][j] == B::W {
                break;
            }
            b[i][j] = B::L;
        }
        for j in (0..j).rev() {
            if b[i][j] == B::W {
                break;
            }
            b[i][j] = B::L;
        }
        for j in j..w {
            if b[i][j] == B::W {
                break;
            }
            b[i][j] = B::L;
        }
    }
    let mut r = 0;
    for i in 0..h {
        for j in 0..w {
            if b[i][j] == B::L {
                r += 1;
            }
        }
    }

    println!("{}", r);
}
