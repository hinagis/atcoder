fn main() {
    proconio::input! {
        n: usize,
        a: [u64; 1 << n]
    }
    let mut q = Vec::with_capacity(1 << n);
    for i in 0..(1 << n) {
        q.push(i);
    }
    while q.len() > 2 {
        let mut nq = Vec::with_capacity(q.len() / 2);
        for i in (0..q.len()).step_by(2) {
            nq.push(if a[q[i]] > a[q[i + 1]] {q[i]} else {q[i + 1]});
        }
        q = nq;
    }

    println!("{}", if a[q[0]] < a[q[1]] {q[0]} else {q[1]} + 1);
}
