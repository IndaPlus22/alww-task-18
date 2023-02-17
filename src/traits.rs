pub trait DasHash {
    fn hash(self) -> usize;
}

impl DasHash for &str {
    fn hash(self) -> usize {
        let mut hash = self.as_bytes();
        // .iter()
        // .fold(0, |acc, x| acc + ((*x) as usize * 17))
        // .overflowing_pow(2)
        // .0;
        let mut sum: usize = 0;
        for x in hash {
            sum += ((*x) as usize * 17);
            sum = sum.overflowing_pow(2).0;
        }
        let mut hash = sum;
        while hash < 100000000 {
            hash = hash.overflowing_pow(2).0;
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
