//use proconio::input;
use std::io::stdin;

fn main() {
    // input! { n: u8, _q: usize }
    let n: u8 = read().parse().unwrap();

    let mut ans = Vec::with_capacity(n as usize);
    if n == 5 {
        let g1 = if ask_lt("A", "B") { ("A", "B") } else { ("B", "A") };
        let g2 = if ask_lt("C", "D") { ("C", "D") } else { ("D", "C") };
        let (g1, g2) = if ask_lt(g1.0, g2.0) { (g1, g2) } else { (g2, g1) };
        ans.push(g1.0.to_string());
        ans.push(g2.0.to_string());
        ans.push(g2.1.to_string());
        let e = bs(&ans, &"E", 0, ans.len());
        ans.insert(e, "E".to_string());
        ans.insert(bs(&ans, g1.1, if e == 0 { 2 } else { 1 }, ans.len()), g1.1.to_string());
    } else { // n == 26
        ans.push("A".to_string());
        let mut ls = Vec::with_capacity(n as usize - 1);
        for i in 1..n {
            ls.push((('A' as u8 + i) as char).to_string());
        }
        for t in ls {
            ans.insert(bs(&ans, &t, 0, ans.len()), t.to_string());
        }
    }

    println!("! {}", ans.join(""));
}

fn bs(ar: &Vec<String>, t: &str, mut l: usize, mut r: usize) -> usize {
    while l < r {
        let m = (l + r) / 2;
        if ask_lt(&ar[m], t) {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn ask_lt(a: &str, b: &str) -> bool {
    println!("? {} {}", a, b);
    // input! { c: String }
    let c = read();
    c == "<"
}

fn read() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.split_whitespace().next().unwrap().to_string()
}
