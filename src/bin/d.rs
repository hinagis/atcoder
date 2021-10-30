use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize}
    let mut d = vec![(0, 0); n];
    for i in 0..n {
        d[i].0 = i;
        d[i].1 = i;
    }
    for _ in 0..q {
        input! {k: u8}
        input! {x: Usize1}
        if k == 3 {
            let mut c = 1;
            let mut s = x;
            while d[s].0 != s {
                c += 1;
                s = d[s].0;
            }
            let mut e = x;
            while d[e].1 != e {
                c += 1;
                e = d[e].1;
            }
            print! {"{}", c};
            let mut i = s;
            while i != e {
                print! {" {}", i + 1};
                i = d[i].1;
            }
            println!(" {}", e + 1);
        } else {
            input! {y: Usize1}
            if k == 1 {
                d[x].1 = y;
                d[y].0 = x;
            } else {
                d[x].1 = x;
                d[y].0 = y;
            }
        }
    }
}
