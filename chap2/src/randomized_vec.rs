use crate::xor64::XOR64;

const NUM: usize = 20_000_000;

pub fn randomized_vec(seed: u64) -> Vec<u64> {
    let mut generator = XOR64::new(seed);

    let mut v = Vec::new();
    for _ in 0..NUM {
        let x = generator.next();
        v.push(x);
    }
    return v;
}
