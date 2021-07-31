use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {q: usize}

    let mut b = 0i64;
    let mut v = vec![];
    for _ in 0..q {
        input! {p: u8}
        if p == 3 {
            println!("{}", v.remove(0) + b);
        } else {
            input! {x: i64}
            if p == 1 {
                let i = match v.binary_search(&(x - b)) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                v.insert(i, x - b);
            } else {
                b += x;
            }
        }
    }
}
