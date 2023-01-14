use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        mut t: C
    }

    let b = t.drain(n..).rev().collect::<Vec<_>>();
    let ztb = z(&t.iter().chain(b.iter()).collect::<Vec<_>>());
    let zbt = z(&b.iter().chain(t.iter()).collect::<Vec<_>>());
    for i in 0..n {
        if ztb[2 * n - i] == i && zbt[n + i] == n - i {
            println!("{}", t[0..i]
                .iter()
                .chain(t[i..].iter().rev())
                .collect::<String>()
            );
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}

fn z(s: &Vec<&char>) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n + 1];
    res[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
