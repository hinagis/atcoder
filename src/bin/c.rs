use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
    }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = i + 1;
    }

    println!("{}", a.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
    while plus(&mut a, m) {
        println!("{}", a.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
    }
}

fn plus(a: &mut Vec<usize>, m: usize) -> bool {
    let mut b = a.clone();
    let mut i = a.len() - 1;
    while i > 0 {
        if b[i] < m - (a.len() - (i + 1)) {
            break;
        }
        i -= 1;
    }

    if b[i] >= m - (a.len() - (i + 1)) {
        return false;
    }

    b[i] += 1;
    i += 1;
    dbg!(&b, &i);
    while i < b.len() {
        b[i] = b[i - 1] + 1;
        i += 1;
    }
    *a = b;
    return true;
}
