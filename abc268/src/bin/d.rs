use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m]
    }

    let mut h = std::collections::HashSet::new();
    for t in t {
        h.insert(t);
    }

    let mut c = 0;
    for i in 0..n {
        c += s[i].len();
    }

    if c + n - 1 > 16 {
        println!("-1");
        return;
    }

    if n == 1 {
        if s[0].len() < 3 || s[0].len() > 16 || h.contains(&s[0]) {
            println!("-1");
        } else {
            println!("{}", s[0]);
        }
        return;
    }

    for perm in s.iter().permutations(n) {
        for i in 0..=(16 - c + 1 - n) {
            for j in (0..n - 1).combinations_with_replacement(i) {
                let mut uc = vec![1; n - 1];
                for k in j {
                    uc[k] += 1;
                }

                let mut ans = perm[0].clone();
                for k in 1..n {
                    let u = "_".repeat(uc[k - 1]);
                    let u = u.as_str();
                    ans += u;
                    ans += perm[k];
                }

                if ! h.contains(&ans) {
                    println!("{}", ans);
                    return;
                }
            }
        }
    }

    println!("-1");
}
