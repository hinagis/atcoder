use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {n: u32}

    if n % 2 == 0 {
        let n2 = n / 2;
        let mi = (0..n2).fold(0, |s, _| (s << 2) | 1);
        let mj = 1 << (n - 1);
        let mut i: u32 = 1 << n2 - 1;
        while i <= mi {
            if i.count_ones() == n2 {
                let mut j = mj;
                let mut s = String::new();
                let mut c = 0;
                while j > 0 {
                    if i & j == 0 {
                        c += 1;
                        s += "(";
                    } else {
                        if c == 0 {
                            break;
                        }
                        c -= 1;
                        s += ")";
                    }
                    j >>= 1;
                }
                if j == 0 {
                    println!("{}", s);
                }
            }
            i += 1;
        }
    }
}
