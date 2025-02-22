use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let mut p = vec![];
    for &c in &s {
        if c == '(' || c == '[' || c == '<' {
            p.push(c);
            continue;
        }
        if let Some(l) = p.pop() {
            if (l == '(' && c == ')')
            || (l == '[' && c == ']')
            || (l == '<' && c == '>') {
                continue;
            } else {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("{}", if p.is_empty() { "Yes" } else { "No" });
}
