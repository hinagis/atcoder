//use proconio::input;
use std::io::stdin;

fn main() {
    // input! { n: u8, _q: usize }
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut it = buf.split_whitespace();
    let n: u8 = it.next().unwrap().parse().unwrap();
    let _q = it.next().unwrap();

    let mut ans = vec![];
    ans.push("A".to_string());
    let mut ls = vec![];
    for i in 1..n {
        ls.push((('A' as u8 + i) as char).to_string());
    }
    for t in ls {
        let mut l = 0;
        let mut r = ans.len();
        while l < r {
            let m = (l + r) / 2;
            println!("? {} {}", ans[m], t);
//            input! { c: String }
            let mut buf = String::new();
            stdin().read_line(&mut buf).ok();
            let mut it = buf.split_whitespace();
            let c = it.next().unwrap();
            if c == "<" {
                l = m + 1;
            } else {
                r = m;
            }
        }
        ans.insert(l, t.to_string());
    }

    println!("! {}", ans.join(""));
}
