use proconio::input;

const LIM: usize = 1000000;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();

    let mut f = vec![true; LIM + 1];
    let mut c = vec![0; LIM + 1];
    let mut r = 0;
    for i in 0..n {
        if f[a[i]] || c[a[i]] > 0 {
            c[a[i]] += 1;
        }
        let mut x = 1;
        if f[a[i]] {
            while a[i] * x <= LIM {
                f[a[i] *x] = false;
                x += 1;
            }
            r += 1;
        }
    }
    println!("{}", r - c.iter().filter(|&ci| *ci > 1).count());
}
