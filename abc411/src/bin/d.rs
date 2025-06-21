use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
    }
    let mut srv = vec![(0, 0, 0)];
    let mut pcs = vec![vec![((0, 0, 0), String::new())]; n];
    for _ in 0..q {
        I! {
            t: u8,
            p: U
        }
        match t {
            1 => {
                pcs[p].push((srv.last().unwrap().clone(), String::new()));
            },
            2 => {
                I! {s: String}
                pcs[p].last_mut().unwrap().1.push_str(&s.clone());
            },
            _ => {
                let ((_, _, l), s) = pcs[p].last().unwrap();
                srv.push((p, pcs[p].len() - 1, l + s.len()));
            }
        }
    }
    fn dfs(pcs: &Vec<Vec<((usize, usize, usize), String)>>,p: usize, r: usize, l: usize) -> String {
        if p == 0 && r == 0 {
            pcs[p][r].1.get(0..l).unwrap().to_string()
        } else {
            let mut pt = dfs(&pcs, pcs[p][r].0.0, pcs[p][r].0.1, pcs[p][r].0.2);
            let t = l - pt.len();
            pt.push_str(pcs[p][r].1.get(0..t).unwrap());
            pt
        }
    }
    let r = dfs(&pcs, srv.last().unwrap().0, srv.last().unwrap().1, srv.last().unwrap().2);
    if r.len() > 0 {
        println!("{r}");
    }
}
