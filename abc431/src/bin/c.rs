use proconio::input as I;

fn main() {
    I! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [u32; n],
        mut b: [u32; m]
    }
    h.sort();
    b.sort();

    let mut j = 0;
    for i in 0..k {
        while j < m && b[j] < h[i] {
            j += 1;
        }
        if j >= m {
            println!("No");
            return;
        }
        j += 1;
    }
    println!("Yes");
}
