use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        a: [U; n]
    }

    let mut f = vec![false; n];
    for &i in &a {
        f[i] = true;
    }
    let mut h = vec![];
    for i in 0..n {
        if f[i] {
            h.push(i);
        }
    }

    let mut d = vec![false; n];
    let mut l = vec![false; n];
    for &i in &h {
        let mut f = vec![false; n];
        let mut j = a[i];
        while ! f[j] {
            if d[j] || l[j] {break}
            d[j] = true;
            f[j] = true;
            j = a[j];
        }
        if ! f[j] || l[j] {continue}

        let s = j;
        l[s] = true;
        j = a[j];
        while j != s {
            l[j] = true;
            j = a[j];
        }
    }

    for i in 0..n {
        if d[i] || l[i] {continue}

        let mut f = vec![false; n];
        let mut j = a[i];
        while ! f[j] {
            if l[j] {break}
            f[j] = true;
            j = a[j];
        }
        if l[j] {continue}

        let s = j;
        l[s] = true;
        j = a[j];
        while j != s {
            l[j] = true;
            j = a[j];
        }
    }

    println!("{}", l.iter().filter(|&&f| f).count());
}
