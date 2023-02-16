pub trait DasHash {
    fn hash(self) -> usize;
}

impl DasHash for &str {
    fn hash(self) -> usize {
        let mut hash = self
            .as_bytes()
            .iter()
            .fold(0, |acc, x| acc + acc + ((*x) as usize * 17))
            .overflowing_pow(2)
            .0;
        while hash < 10000000 {
            hash = hash.overflowing_mul(23).0;
        }
        let hash = hash.to_string();
        let hash: usize = hash
            .get((hash.len() / 4)..(hash.len() - hash.len() / 4))
            .unwrap()
            .parse()
            .unwrap();
        hash
    }
}
