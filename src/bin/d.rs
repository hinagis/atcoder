use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if m == 0 {
        println!("1");
    } else {
        let mut ff = vec![false; n];
        let mut f = vec![vec![]; n];
        for _ in 0..m {
            input!{ a: Usize1, b: Usize1 }
            f[a].push(b);
            f[b].push(a);
            ff[a] = true;
            ff[b] = true;
        }
    
        let mut mm = 0;
        for i in 0..n {
            if ff[i] {
                ff[i] = false;
                let mut g = 1;
                let mut q = vec![i];
                while let Some(qi) = q.pop() {
                    for &j in &f[qi] {
                        if ff[j] {
                            ff[j] = false;
                            g += 1;
                            q.push(j);
                        }
                    }
                }
                mm = mm.max(g);
            }
        }
        println!("{}", mm);
    }
}
