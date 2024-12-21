use proconio::input as I;

fn main() {
    I! {
        t: String,
        n: usize,
    }

    let mut d = std::collections::HashMap::new();
    d.insert(String::new(), 0);
    for _ in 0..n {
        I! {
            a: usize,
            s: [String; a],
        }
        let mut e = d.clone();
        for (k, v) in d.iter() {
            for s in s.iter() {
                let s = k.clone() + s;
                if t.starts_with(&s) {
                    if let Some(e) = e.get_mut(&s) {
                        *e = (*e).min(v + 1);
                    } else {
                        e.insert(s, v + 1);
                    }
                }
            }
        }
        d = e;
    }
    println!("{}", if let Some(v) = d.get(&t) {*v} else {-1});
}
