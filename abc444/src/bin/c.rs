use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {
        n: usize,
        mut a: [u32; n]
    }
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    a.sort_by(|a, b| b.cmp(a));

    let mut v = vec![];

    let mut l = 1;
    let mut r = n - 2;
    let mut f = true;
    while l < r {
        if a[l] + a[r] != a[0] + a[n - 1] {
            f = false;
            break;
        }
        l += 1;
        r -= 1;
    }
    if f && l != r{
        v.push(a[0] + a[n - 1]);
    }

    let mut l = 1;
    let mut r = n - 1;
    let mut f = true;
    while l <= r && a[l] == a[0] {
        l += 1;
    }
    while l < r {
        if a[l] + a[r] != a[0] {
            f = false;
            break;
        }
        l += 1;
        r -= 1;
    }
    if f && l != r{
        v.push(a[0]);
    }
    v.sort();
    println!("{}", v.iter().join(" "));
}
