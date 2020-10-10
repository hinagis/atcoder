use proconio::input;

fn main() {
    input! {n: usize}
    let mut e = vec![(None, None); n * 2];
    for i in 0..n {
        input! {a: isize, b: isize}
        let a = if a >= 0 {Some(a as usize - 1)} else {None};
        let b = if b >= 0 {Some(b as usize - 1)} else {None};
        if let Some(a) = a {
            if e[a].0 != None {ng()}
            e[a].0 = Some((true, i));
        }
        if let Some(b) = b {
            if e[b].0 != None {ng()}
            e[b].0 = Some((false, i));
        }
        if let (Some(a), Some(b)) = (a, b){
            if a > b {ng()}
            e[a].1 = Some(b);
            e[b].1 = Some(a);
        }
    }

    let mut dp = vec![false; n * 2 + 1];
    dp[0] = true;
    for i in 0..(n * 2) {
        if dp[i] {
            for j in ((i + 1)..(n * 2)).step_by(2) {
                let mut ok = true;
                let w = (j - i + 1) / 2;
                for p in i..(i + w) {
                    let q = p + w;
                    if let Some(ep) = e[p].0 {ok &= ep.0}
                    if let Some(eq) = e[q].0 {ok &= !eq.0}
                    if let (Some(ep), Some(eq)) = (e[p].0, e[q].0) {ok &= ep.1 == eq.1}
                    if let Some(ep) = e[p].1 {ok &= ep == q}
                    if let Some(eq) = e[q].1 {ok &= eq == p}
                }
                dp[j + 1] |= ok;
            }
        }
    }
    println!("{}", if dp[n * 2] {"Yes"} else {"No"});
}

fn ng() {
    println!("No");
    std::process::exit(0);
}