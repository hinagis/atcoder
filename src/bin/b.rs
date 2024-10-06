fn main() {
    proconio::input! {
        s: String,
        t: String
    }
    fn calc(s: &Vec<char>, t: &Vec<char>) -> usize {
        let n = s.len().min(t.len());
        for i in 0..n {
            if s[i] != t[i] {
                return i + 1;
            }
        }
        n + 1
    }
    println!("{}", if s == t {0} else {calc(&s.chars().collect(), &t.chars().collect())});
}
