use proconio::{input, marker::Usize1};

fn chmax(a: &mut Option<u32>, b: u32) {
    *a = Some(if let Some(v) = *a {
        v.max(b)
    } else {
        b
    });
}

fn rotate(x: &mut usize, y: &mut usize, z: &mut usize) {
    let w = *x;
    *x = *y;
    *y = *z;
    *z = w;
}

fn get_v(m: u32, l: Option<u32>) -> u32 {
    if let Some(l) = l {
        m.max(l + 1)
    } else {
        m
    }
}

struct DP {
    l12: Vec<Vec<Option<u32>>>,
    max: (u32, Vec<Option<u32>>),
    n: usize,
}

impl DP {
    fn new(n: usize) -> DP {
        DP {
            l12: vec![vec![None; n]; n],
            max: (0, vec![None; n]),
            n: n,
        }
    }

    fn set(&mut self, l1: usize, l2: usize, v: u32) {
        if l2 != self.n {
            chmax(&mut self.l12[l1][l2], v);
            chmax(&mut self.l12[l2][l1], v);
            chmax(&mut self.max.1[l2], v);
        }
        chmax(&mut self.max.1[l1], v);
        self.max.0 = self.max.0.max(v);
    }
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n * 3],
    }

    let mut dp = DP::new(n);
    dp.set(a[0], a[1], 0);
    let mut c = 0; // counter, number of cuts 3 to 5 from the left.
    for i in 1..n {
        let (mut x, mut y, mut z) = (a[i * 3 - 1], a[i * 3], a[i * 3 + 1]);
        if x == y && x == z {
            c += 1;
        } else {
            let m = dp.max.0;
            let mut q = vec![];
            for _ in 0..3 {
                for l in 0..n {
                    if let Some(m) = dp.max.1[l] {
                        let v = if y == z { get_v(m, dp.l12[l][y]) } else { m };
                        q.push((l, x, v));
                    }
                }
                let v = if y == z { get_v(m, dp.max.1[y]) } else { m };
                q.push((x, n, v));

                let v = get_v(m, dp.l12[z][z]);
                q.push((x, y, v));

                rotate(&mut x, &mut y, &mut z);
            }

            for (x, y, v) in q {
                dp.set(x, y, v);
            }
        }
    }

    let l = a[3 * n - 1];
    let v = get_v(dp.max.0, dp.l12[l][l]);
    println!("{}", v + c);
}
