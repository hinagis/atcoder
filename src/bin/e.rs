use proconio::input;

const A: usize = 1000_000;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ma = 0;
    let mut ac = vec![0; A + 1];
    for &a in &a {
        ac[a] += 1;
        ma = ma.max(a);
    }

    let mut pc = true;
    for i in (0..=ma).skip(2) {
        let mut c = 0;
        for j in (0..=ma).skip(i).step_by(i) {
            c += ac[j];
        }
        if c > 1 {
            pc = false;
            break;
        }
    }

    println!("{} coprime", if pc {
        "pairwise"
    } else {
        let mut v = a[0];
        for i in 1..n {
            v = num_integer::gcd(a[i], v);
        }
        if v == 1 {
            "setwise"
        } else {
            "not"
        }
    });
}
