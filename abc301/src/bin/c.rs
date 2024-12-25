fn main() {
    proconio::input! {
        s: String,
        t: String
    }
    let mut a = [0; 27];
    for c in s.bytes() {
        if c == b'@' {
            a[26] += 1;
        } else {
            a[(c - b'a') as usize] += 1;
        }
    }
    let mut b = [0; 27];
    for c in t.bytes() {
        if c == b'@' {
            b[26] += 1;
        } else {
            b[(c - b'a') as usize] += 1;
        }
    }

    for i in 0..26 {
        if match (i as u8 + b'a') as char {'a'| 't'| 'c'| 'o'| 'd'| 'e'| 'r' => true, _ => false} {
            if a[i] <= b[i] {
                let d = b[i] - a[i];
                if a[26] >= d {
                    a[26] -= d;
                } else {
                    println!("No");
                    return;
                }
            } else {
                let d = a[i] - b[i];
                if b[26] >= d {
                    b[26] -= d;
                } else {
                    println!("No");
                    return;
                }
            }
        } else {
            if a[i] != b[i] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
