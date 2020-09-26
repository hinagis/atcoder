use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();

    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        if a[i] == b[j] {
            println!("{}", a[i]);
            i = skip(&a, i, n);
            j = skip(&b, j, j);
        } else if a[i] < b[j] {
            i = skip(&a, i, n);
        } else {
            j = skip(&b, j, m);
        }
    }
}

fn skip(a: &Vec<usize>, mut i: usize, n: usize) -> usize {
    let t = a[i];
    while i < n && a[i] == t {
        i += 1;
    }
    i
}
