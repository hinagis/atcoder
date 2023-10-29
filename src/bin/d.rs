use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {
        n: u32,
        s: B
    }
    let m = 10u64.pow(n);

    let mut t = [0u8; 10];
    for c in s {
        t[(c - b'0') as usize] += 1;
    }

    let mut c = 0u64;
    for i in 0.. {
        let mut i = i * i;
        if i >= m {break}

        let mut j = [0; 10];
        for _ in 0..n {
            j[(i % 10) as usize] += 1;
            i /= 10;
        }

        if j == t {
            c += 1;
        }
    }
    println!("{c}");
}
