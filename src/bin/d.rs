use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    }
    let v = calc(&a, &h, &w, 0, 0, 0, vec![0, 0]);
    println!("{}", if v[0] == v[1] {"Draw"} else if v[0] > v[1] {"Takahashi"} else {"Aoki"});
}

fn calc(a: &Vec<Vec<char>>, h: &usize, w: &usize, i: usize, j: usize, p: usize, v: Vec<i64>) -> Vec<i64> {
    if i >= h - 1 && j >= w - 1 {
        v
    } else {
        if i < h - 1 {
            let mut ri = v.clone();
            let i = i + 1;
            ri[p] += if a[i][j] == '+' {1} else {-1};
            ri = calc(a, h, w, i, j, p ^ 1, ri);
            if ri[p] > ri[p ^ 1] || j >= w - 1 {
                ri
            } else {
                let mut rj = v.clone();
                let i = i - 1;
                let j = j + 1;
                rj[p] += if a[i][j] == '+' {1} else {-1};
                rj = calc(a, h, w, i, j, p ^ 1, rj);
                if rj[p] > rj[p ^ 1] || ri[p] != ri[p ^ 1] {
                    rj
                } else {
                    ri
                }
            }
        } else {
            let mut rj = v.clone();
            let j = j + 1;
            rj[p] += if a[i][j] == '+' {1} else {-1};
            calc(a, h, w, i, j, p ^ 1, rj)
        }
    }
}
