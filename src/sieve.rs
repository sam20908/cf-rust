pub struct Sieve {
    primes: Vec<usize>,
    prime_divisor: Vec<usize>,
}
impl Sieve {
    pub fn new(n: usize) -> Sieve {
        let mut primes = Vec::new();
        let mut prime_divisor: Vec<usize> = (0..n + 1).collect();
        for i in 2..n + 1 {
            if prime_divisor[i] == i {
                primes.push(i);
            }
            for j in 0..primes.len() {
                if i * primes[j] > n {
                    break;
                }
                prime_divisor[i * primes[j]] = primes[j];
                if i % primes[j] == 0 {
                    break;
                }
            }
        }
        Sieve {
            primes,
            prime_divisor,
        }
    }
    pub fn primes(&self) -> &Vec<usize> {
        &self.primes
    }
    pub fn prime_divisor(&self, x: usize) -> usize {
        self.prime_divisor[x]
    }
    pub fn is_prime(&self, x: usize) -> bool {
        x > 1 && self.prime_divisor[x] == x
    }
}
