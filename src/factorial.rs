pub struct Factorial<const M: i64> {
    fact: Vec<i64>,
    fact_inv: Vec<i64>,
}

impl<const M: i64> Factorial<M> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![1; n];
        let mut inv = vec![1; n];
        let mut fact_inv = vec![1; n];
        for i in 2..n + 1 {
            fact[i] = fact[i - 1] * i as i64 % M;
            inv[i] = inv[(M % i as i64) as usize] * (M - M / i as i64) % M;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % M;
        }
        Self { fact, fact_inv }
    }
    pub fn ncr(&self, n: usize, k: usize) -> i64 {
        self.fact[n] * self.fact_inv[k] % M * self.fact_inv[n - k] % M
    }
}
