use itertools::Itertools;

fn main() {
    proconio::input! {s: [u32; 5]}
    let mut r = vec![];
    for i in 0..5 {
        let mut ti = (String::new(), 0);
        ti.0.push((b'A' + i as u8) as char);
        ti.1 += s[i];
        r.push(ti.clone());
        for j in i + 1..5 {
            let mut tj = ti.clone();
            tj.0.push((b'A' + j as u8) as char);
            tj.1 += s[j];
            r.push(tj.clone());
            for k in j + 1..5 {
                let mut tk = tj.clone();
                tk.0.push((b'A' + k as u8) as char);
                tk.1 += s[k];
                r.push(tk.clone());
                for l in k + 1..5 {
                    let mut tl = tk.clone();
                    tl.0.push((b'A' + l as u8) as char);
                    tl.1 += s[l];
                    r.push(tl.clone());
                    for m in l + 1..5 {
                        let mut tm = tl.clone();
                        tm.0.push((b'A' + m as u8) as char);
                        tm.1 += s[m];
                        r.push(tm.clone());
                    }
                }
            }
        }
    }
    r.sort_by(|a, b| if a.1 == b.1 {a.0.cmp(&b.0)} else {b.1.cmp(&a.1)});

    println!("{}", r.iter().map(|(k, _)| k).join("\n"));
}
