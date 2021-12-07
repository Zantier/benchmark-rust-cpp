const N: i32 = 10;
struct Perm {
    res: Vec<String>,
}
impl Perm {
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
            let mut s2 = s.clone();
            s2.push(c);
            self.rec(done|1<<i, s2);
        }
    }
}
fn main() {
    let mut perm = Perm {
        res: Vec::new(),
    };

    perm.rec(0, String::from(""));
    println!("Size: {}", perm.res.len());
    for s in perm.res.iter().take(5) {
        println!("{}", s);
    }
}
