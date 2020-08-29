use proconio::{input, marker::Usize1};

fn chmax(a: &mut i32, b: i32) { *a = (*a).max(b); }
fn rotate(x: &mut usize, y: &mut usize, z: &mut usize) {
    let w = *x;
    *x = *y;
    *y = *z;
    *z = w;
}

struct DP {
    l12: Vec<Vec<i32>>,
    l1_max: Vec<i32>,
    l2_max: Vec<i32>,
    max: i32,
    n: usize,
}

impl DP {
    fn new(n: usize) -> DP {
        DP {
            l12: vec![vec![i32::min_value(); n + 1]; n + 1],
            l1_max: vec![i32::min_value(); n],
            l2_max: vec![i32::min_value(); n],
            max: i32::min_value(),
            n: n,
        }
    }
    fn set(&mut self, l1: usize, l2: usize, v: i32) {
        if l2 != self.n {
            chmax(&mut self.l12[l1][l2], v);
            chmax(&mut self.l12[l2][l1], v);
            chmax(&mut self.l1_max[l2], v);
            chmax(&mut self.l2_max[l2], v);
        }
        chmax(&mut self.l1_max[l1], v);
        chmax(&mut self.l2_max[l1], v);
        chmax(&mut self.max, v);
    }
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n * 3],
    }

    let mut dp = DP::new(n);
    dp.set(a[0], a[1], 0);
    let mut mc = 0;
    for i in 1..n {
        let (mut x, mut y, mut z) = (a[i * 3 - 1], a[i * 3], a[i * 3 + 1]);
        if x == y && x == z {
            mc += 1;
        } else {
            let mut q = vec![];
            for _ in 0..3 {
                for l in 0..n {
                    let mut v = dp.l1_max[l];
                    if y == z {
                        v = v.max(dp.l12[y][l] + 1);
                    }
                    q.push((x, l, v));
                }
                let mut v = dp.max;
                if y == z {
                    v = v.max(dp.l1_max[y] + 1);
                }
                q.push((x, n, v));

                let v = dp.max.max(dp.l12[z][z] + 1);
                q.push((x, y, v));
                rotate(&mut x, &mut y, &mut z);
            }
            for (x, y, v) in q {
                dp.set(x, y, v);
            }
        }
    }
    let l = a[3 * n - 1];
    let v = dp.max.max(dp.l12[l][l] + 1);
    println!("{}", v + mc);
}
