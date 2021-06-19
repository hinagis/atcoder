const L: usize = 200001;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }

    let mut c = 0;

    let mut f = [false; L];
    for i in 0..n {
        if f[a[i]] == false {
            f[a[i]] = true;
            c += 1;
        }
    }

    let mut g = vec![vec![]; L];
    let mut p = 0;
    let mut q = n - 1;
    while p < q  {
        g[a[p]].push(a[q]);
        g[a[q]].push(a[p]);
        p += 1;
        q -= 1;
    }

    for i in 1..L {
        if f[i] {
            c -= 1;
            dfs(&mut f, &g, i);
        }
    }

    println!("{}", c);
}

fn dfs(f: &mut [bool], g: &Vec<Vec<usize>>, i: usize) {
    if f[i] == false {
        return;
    }
    f[i] = false;
    for &j in &g[i] {
        dfs(f, g, j);
    }
}
