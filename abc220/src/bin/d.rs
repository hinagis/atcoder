const M: u64 = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut b = [0; 11];
    b[a[0]] += 1;
    for i in 1..n {
        let mut c = [0; 11];
        for j in 0..=10 {
            let (u, v) = ((j + a[i]) % 10, (j * a[i]) % 10);
            c[u] += b[j]; c[u] %= M;
            c[v] += b[j]; c[v] %= M;
        }
        b = c;
    }

    for i in 0..10 {
        println!("{}", b[i]);
    }
}
