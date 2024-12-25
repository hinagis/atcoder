use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        mut sc: [(String, u64); n],
    }

    let mut rev = Vec::new();
    let mut cost = u64::max_value();
    let sc: Vec<_> = sc
        .iter()
        .filter(|e| {
            let r = e.0.chars().rev().collect::<String>();
            let f = e.0 != r;
            if f {
                rev.push(r);
            } else {
                cost = cost.min(e.1);
            }
            f
        })
        .collect();

    let solver = Solver::new(&sc, &rev);
    for e in &sc {
        cost = cost.min(solver.clone().calc(e.0.as_str(), e.1, true));
    }

    if cost < u64::max_value() {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}

#[derive(Clone)]
struct Solver<'a> {
    done: HashMap<(&'a str, bool), u64>,
    n: usize,
    sc: &'a Vec<&'a (String, u64)>,
    rev: &'a Vec<String>,
}

impl<'a> Solver<'a> {
    fn new(sc: &'a Vec<&'a (String, u64)>, rev: &'a Vec<String>) -> Solver<'a> {
        Solver {
            done: HashMap::new(),
            n: sc.len(),
            sc: sc,
            rev: rev,
        }
    }

    fn calc(&mut self, s: &'a str, cost: u64, is_left: bool) -> u64 {
        let mut v = u64::max_value();
        if if let Some(&c) = self.done.get(&(s, is_left)) { cost >= c } else { false } {
            v
        } else {
            self.done.insert((s, is_left), cost);
            for i in 0..self.n {
                let oppo = if is_left { &self.rev[i] } else { &self.sc[i].0 };
                let next = if s.starts_with(oppo) {
                    Some((&s[oppo.len()..], is_left))
                } else if oppo.starts_with(&s) {
                    Some((&oppo[s.len()..], !is_left))
                } else {
                    None
                };
                if let Some((next, is_left)) = next {
                    let cost = cost + self.sc[i].1;
                    v = v.min(if next == next.chars().rev().collect::<String>() {
                        cost
                    } else {
                        self.calc(next, cost, is_left)
                    });
                }
            }
            v
        }
    }
}
