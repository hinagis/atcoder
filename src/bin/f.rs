use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        sc: [(String, u64); n],
    }

    let rev: Vec<_> = sc
        .iter()
        .map(|(s, _)| s.chars().rev().collect::<String>())
        .collect();

    let mut cost = u64::max_value();
    for i in 0..n {
        let mut solver = Solver::new(n, &sc, &rev);
        cost = cost.min(if sc[i].0 == solver.rev[i] {
            sc[i].1
        } else {
            solver.calc(sc[i].0.as_str(), sc[i].1, true)
        });
    }

    if cost < u64::max_value() {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}

struct Solver<'a> {
    done: HashMap<(&'a str, bool), u64>,
    n: usize,
    sc: &'a Vec<(String, u64)>,
    rev: &'a Vec<String>,
}

impl<'a> Solver<'a> {
    fn new(n: usize, sc: &'a Vec<(String, u64)>, rev: &'a Vec<String>) -> Solver<'a> {
        Solver {
            done: HashMap::new(),
            n: n,
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
