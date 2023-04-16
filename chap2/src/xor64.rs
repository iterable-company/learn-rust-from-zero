pub struct XOR64 {
    x: u64,
}

impl XOR64 {
    pub fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88_172_645_463_325_252,
        }
    }

    pub fn next(&mut self) -> u64 {
        let x = self.x;
        let x = x ^ (x << 13);
        let x = x ^ (x >> 7);
        let x = x ^ (x << 17);
        self.x = x;
        return x;
    }
}
