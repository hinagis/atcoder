const E: f64 = 1e-6;

fn main() {
    proconio::input! {
        n: usize,
        ab: [(f64, f64); n],
        cd: [(f64, f64); n],
    }

    if n == 1 {
        println!("Yes");
        return;
    }

    let g = ab.iter().fold((0f64, 0f64), |g, &(x, y)| (g.0 + x, g.1 + y));
    let mut ab = ab.iter().map(|(x, y)| (x * n as f64 - g.0, y * n as f64 - g.1)).collect::<Vec<_>>();
    for i in 0..n {
        if ab[i].0 != 0f64 || ab[i].1 != 0f64 {
            ab.swap(0, i);
            break;
        }
    }

    let g = cd.iter().fold((0f64, 0f64), |g, &(x, y)| (g.0 + x, g.1 + y));
    let cd = cd.iter().map(|(x, y)| (x * n as f64 - g.0, y * n as f64 - g.1)).collect::<Vec<_>>();

    for i in 0..n {
        let t = cd[i].1.atan2(cd[i].0) - ab[0].1.atan2(ab[0].0);

        let mut f = true;
        for j in 0..n {
            let a = ab[j].0 * t.cos() - ab[j].1 * t.sin();
            let b = ab[j].0 * t.sin() + ab[j].1 * t.cos();

            let mut f2 = false;
            for k in 0..n {
                if (a - cd[k].0).abs() <= E && (b - cd[k].1).abs() <= E {
                    f2 = true;
                }
            }
            f &= f2;
        }

        if f {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
