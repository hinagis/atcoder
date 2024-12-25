fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        xy: [(i32, i32); n]
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut f = vec![vec![true; n]; n];
    for i in 0..n {
        f[i][i] = false;
    }
    let mut c = 0;
    for i in 0..n {
        for j in i + 1..n {
            if f[i][j] {
                f[i][j] = false;
                f[j][i] = false;
                let mut r = vec![i, j];
                let mut p = 2;
                for k in j + 1..n {
                    if chk(&xy, i, j, k) {
                        for &r in &r {
                            f[r][k] = false;
                            f[k][r] = false;
                        }
                        r.push(k);
                        p += 1;
                    }
                }
                if p >= k {
                    c += 1;
                }
            }
        }
    }
    println!("{}", c);
}

fn chk(xy: &Vec<(i32, i32)>, a: usize, b: usize, c: usize) -> bool {
	let l = (xy[b].1 - xy[a].1) * (xy[c].0 - xy[a].0);
	let r = (xy[b].0 - xy[a].0) * (xy[c].1 - xy[a].1);
	return l == r;
}