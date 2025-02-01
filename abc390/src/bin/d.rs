fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }
    let mut h = vec![];
    dfs(&a, &mut h, &mut vec![], 0);
    h.sort_unstable();
    h.dedup();
    println!("{}", h.len());
}

fn dfs(a: &Vec<u64>, h: &mut Vec<u64>, t: &mut Vec<u64>, i: usize) {
    if i == a.len() {
        h.push(t.iter().fold(0, |t, a| t ^ a));
        return;
    }

    for k in 0..t.len() {
        t[k] += a[i];
        dfs(a, h, t, i + 1);
        t[k] -= a[i];
    }
    t.push(a[i]);
    dfs(a, h, t, i + 1);
    t.pop();
}
