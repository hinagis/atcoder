use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = vec![0; a[0]];
    let mut j = a[0] - 1;
    for i in 1..n {
        while j >= a[i] {
            c[j] = i;
            j -= 1;
        }
    }
    for i in 0..m {
        println!("{}", 
            if b[i] >= a[0] {
                1
            } else if b[i] > j {
                c[b[i]] as i32 + 1
            } else {
                -1
            }
        );
    }
}
