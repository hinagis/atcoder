const M: usize = 200_001;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    
    let mut c = vec![0; M];
    for &a in &a {
        c[a] += 1;
    }
    let mut r = 0u64;
    for i in 1..M {
        let mut j = 1;
        while i * j < M {
            r += c[i] * c[j] * c[i * j];
            j += 1;
        }
    }
    println!("{}", r);
}
