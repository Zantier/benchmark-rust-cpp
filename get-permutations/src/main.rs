const N: i32 = 10;
struct Perm {
    res: Vec<Vec<u8>>,
}
impl Perm {
    fn rec(&mut self, done: i32, s: Vec<u8>) {
        if done == (1<<N)-1 {
            self.res.push(s);
            return;
        }

        for i in 0..N {
            if 1<<i&done > 0 {
                continue;
            }

            let c = b'A' + i as u8;
            let mut s2 = Vec::with_capacity(s.len()+1);
            s2.extend_from_slice(&s);
            s2.push(c);
            self.rec(done|1<<i, s2);
        }
    }
}
fn main() {
    let mut perm = Perm {
        res: Vec::new(),
    };

    perm.rec(0, Vec::new());
    println!("Size: {}", perm.res.len());
    for s in perm.res.iter().take(5) {
        println!("{:?}", s);
    }
}
