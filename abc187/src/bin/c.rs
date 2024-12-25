fn main() {
    proconio::input! {
        n: usize,
        s: [String; n]
    }

    let mut h = std::collections::HashMap::new();
    for s in s {
        let w = if s.starts_with('!') {
            s.clone().drain(1..s.len()).collect()
        } else {
            s.clone()
        };
        let p = h.entry(w.clone()).or_insert((false, false));
        if s.starts_with('!') {
            p.0 = true;
        } else {
            p.1 = true;
        }
        if *p == (true, true) {
            println!("{}", w);
            return;
        }
    }
    println!("satisfiable");
}
