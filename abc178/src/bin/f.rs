use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        mut b: [Usize1; n],
    }

    let mut c = vec![0; n];
    let mut d = vec![0; n];
    for i in 0..n {
        c[a[i]] += 1;
        d[b[i]] += 1;
    }

    if c[0] + d[0] > n {
        println!("No");
    } else {
        for i in 1..n {
            if c[i] + d[i] > n {
                println!("No");
                return
            }
            c[i] += c[i - 1];
            d[i] += d[i - 1];
        }

        let mut x = c[0] as isize;
        for i in 1..n {
            x = x.max(c[i] as isize - d[i - 1] as isize)
        }

        if x >= 0 {
            b.rotate_right(x as usize);
        } else {
            b.rotate_left(x.abs() as usize);
        }

        println!("Yes");
        println!("{}", b.iter().map(|v| (v + 1).to_string()).collect::<Vec<String>>().join(" "));
    }
}
