use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: u64,
    }

    let mut ans = vec![];
    let h = trial_division(n);
    for (&i, &ic) in &h {
        let mut new_ans = vec![];
        for j in 1..=ic {
            new_ans.push(i.pow(j));
            for k in 0..ans.len() {
                new_ans.push(i.pow(j) * ans[k]);
            }
        }
        ans.append(&mut new_ans);
    }
    ans.push(1);
    ans.sort();
    for ans in ans {
        println!("{}", ans);
    }
}

fn trial_division(mut n :u64) -> HashMap<u64, u32>{
    let mut primes = HashMap::new();
    let mut i = 2;

    //  n を root n 以下の整数で割り切れるまで割っていく
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            if primes.contains_key(&i) {
                let x = primes.get_mut(&i).unwrap();
                *x += 1;
            } else {
                primes.insert(i, 1);
            }
        }
        i+=1;
    }

    // 最後にnが素数になっている場合はそれ自身も素因数に含めて終了
    if n > 1 {
        if primes.contains_key(&n) {
            let x = primes.get_mut(&n).unwrap();
            *x += 1;
        } else {
            primes.insert(n, 1);
        }
    }
    primes
}
