const N: i32 = 10;
struct Perm {
    res: Vec<String>,
}

fn fact(n: usize) -> usize {
    let mut res = 1;
    for i in 2..n+1 {
        res *= i;
    }
    res
}

impl Perm {

    fn new(n: i32) -> Perm {
        Perm {
            res: Vec::with_capacity(fact(n as usize))
        }
    }

    fn rec(&mut self, done: i32, s: String) {
        if done == (1<<N)-1 {
            self.res.push(s);
            return;
        }

        for i in 0..N {
            if 1<<i&done > 0 {
                continue;
            }

            let c = char::from_u32(b'A' as u32 + i as u32).unwrap();
            let mut s2 = String::with_capacity(s.len() + 1);
            s2.push_str(&s);
            s2.push(c);
            self.rec(done|1<<i, s2);
        }
    }
}
fn main() {
    let mut perm = Perm::new(N);

    perm.rec(0, String::from(""));
    println!("Size: {}", perm.res.len());
    for s in perm.res.iter().take(5) {
        println!("{}", s);
    }
}
