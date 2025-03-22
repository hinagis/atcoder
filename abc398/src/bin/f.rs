use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let mut t = s.clone();
    t.reverse();
    let m = calc(t.iter().collect::<String>());
    if m == t[0..m.len()].iter().collect::<String>() {
        println!("{}{}", s.iter().collect::<String>(), t[m.len()..t.len()].iter().collect::<String>());
    } else {
        println!("{}{}", s.iter().collect::<String>(), t[1..t.len()].iter().collect::<String>());
    }
}

pub fn calc(s: String) -> String {
	let s = format!("|{}", s.chars().map(|c| c.to_string() + "|",).collect::<String>());
	let (mut m, mut e, n) = (0, 0, s.len());
	let mut p = vec![0; n];

	while m < n {
		while m - e > 0
			&& m + e < n - 1
			&& s[m - e - 1..m - e]
				== s[m + e + 1..m + e + 2]
		{
			e += 1;
		}

		p[m] = e;

		let (om, oe,) = (m, e,);
		m += 1;
		e = 0;
		while m <= om + oe {
			let mm = om - (m - om);
			let mme = om + oe - m;

			if p[mm] < mme {
				p[m] = p[mm];
				m += 1;
			} else if p[mm] > mme {
				p[m] = mme;

				m += 1;
			} else {
				e = mme;
				break;
			}
		}
	}

	e = 0;
	for (i, &r,) in p.iter().enumerate() {
		if e < r {
			e = r;
			m = i;
		}
	}

	s[m - e..m + e + 1].chars().filter(|&c| c != '|',).collect()
}
