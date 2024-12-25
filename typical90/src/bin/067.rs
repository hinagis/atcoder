use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
        k: usize
    }
    let mut n = n.iter().map(|v| (v - b'0') as u64).collect::<std::collections::VecDeque<_>>();
    for _ in 0..k {
        let mut d = 0;
        let mut b = 1;
        for &v in n.iter().rev() {
            d += v * b;
            b *= 8;
        }
        n.clear();
        if d == 0 {
            n.push_front(0)
        } else {
            while d > 0 {
                let v = d % 9;
                let v = if v == 8 {5} else {v};
                n.push_front(v);
                d /= 9;
            }
        }
    }
    for &v in &n {
        print!("{}", v);
    }
    println!("");
}
