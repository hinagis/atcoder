fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let (mut l, mut r) = (vec![usize::MAX; n], vec![usize::MAX; n]);
    let p = (0..n).map(|i| calc(&mut l, a[i])).collect::<Vec<_>>();
    let mut q = vec![0; n];
    for i in (0..n).rev() {
        q[i] = calc(&mut r, a[i]);
    }
    println!("{}", (0..n).fold(0, |m, i| m.max(p[i] + q[i] - 1)));
}

fn calc(l: &mut Vec<usize>, a: usize) -> usize{
    l.binary_search(&a)
     .map_or_else(|i| {l[i] = a; i}, |i| i) + 1
}
