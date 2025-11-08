use proconio::{input as I, fastout as F};

#[F]
fn main(){
    const M: i32 = 10i32.pow(9);

    I! {x: [i32]}

    let mut t=std::collections::BTreeMap::new();
    let mut s = {
        let d = x[0];
        t.insert(0, d);
        t.insert(d, d);
        d * 2
    };
    println!("{s}");

    for &x in &x[1..] {
        let d = if let Some((y, p)) = t.range_mut(x..).next() {
            f(&mut s, p, *y - x)
        } else {
            M
        };
        let (y, p) = t.range_mut(..x).last().unwrap();
        let d = d.min(f(&mut s, p, x - *y));
        t.insert(x, d);
        s += d;
        println!("{s}");
    }
}

fn f(s: &mut i32, p: &mut i32, d: i32) -> i32 {
    if *p > d {
        *s -= *p - d;
        *p = d;
    }
    d
}
