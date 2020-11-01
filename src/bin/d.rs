use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {s: Chars}

    let mut cs = vec![0; 10];
    for &c in &s {
        cs[c as usize - '0' as usize] += 1;
    }
    let mut t = vec![];
    for e in (112..1000).step_by(8) {
        let mut h = HashMap::new();
        for c in e.to_string().chars() {
            *h.entry(c).or_insert(0) += 1;
        }
        if h.get(&'0') == None {
            t.push(h)
        }
    }
    if s.len() == 1 {
        if cs[8] == 1 {
            println!("Yes");
            return;
        }
    } else if s.len() == 2 {
        if (cs[1] == 1 && cs[6] == 1)
        || (cs[2] == 1 && cs[4] == 1)
        || (cs[3] == 1 && cs[2] == 1)
        || (cs[4] == 1 && cs[8] == 1)
        || (cs[5] == 1 && cs[6] == 1)
        || (cs[6] == 1 && cs[4] == 1)
        || (cs[7] == 1 && cs[2] == 1)
        || (cs[8] == 2)
        || (cs[9] == 1 && cs[6] == 1) {
            println!("Yes");
            return;
        }
    } else {
        for v in t {
            let mut f = true;
            for e in v {
                if cs[e.0 as usize - '0' as usize] < e.1 {
                    f = false;
                    break;
                }
            }
            if f {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
