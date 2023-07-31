use crate::{random::generator::{Pseudorandom64, IntGenerator}, math::basic::{pow_mod_u32, pow_mod_u64, gcd_u32, gcd_u64}};

/** `PrimalityTest` introduces a trait for primality test. */
pub trait PrimalityTest: Clone {
    /** Test if 32-bit `n` is a prime. */
    fn is_prime_u32(&mut self, n: &u32) -> bool;

    /** Test if 64-bit `n` is a prime. */
    fn is_prime_u64(&mut self, n: &u64) -> bool;
}

/** `FindPrimeFactors` introduces a trait for Extracting prime factors from an integer.*/
pub trait ExtractPrimeFactors: Clone {
    /** Extract all prime factors from an 32-bit integer. */
    fn extract_prime_factors_u32(&mut self, n: &u32) -> Vec<u32>;

    /** Extract all prime factors from an 64-bit integer. */
    fn extract_prime_factors_u64(&mut self, n: &u64) -> Vec<u64>;

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from 32-bit `n`. */
    fn extract_factor_u32(&mut self, n: &u32) -> Option<u32>;

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from 64-bit `n`. */
    fn extract_factor_u64(&mut self, n: &u64) -> Option<u64>;
}

/** `BruteForcePrimalityTest` is a naive algorithm of primality test. Its computation complexity is $O(\sqrt{n})$. */
#[derive(Clone, Debug)]
pub struct BruteForcePrimalityTest {}

impl BruteForcePrimalityTest {
    /** New a `BruteForcePrimalityTest`. */
    pub fn new() -> BruteForcePrimalityTest {
        BruteForcePrimalityTest {}
    }
}

impl PrimalityTest for BruteForcePrimalityTest {
    /** Test if `n` is a prime in $O(\sqrt{n})$ time. */
    fn is_prime_u32(&mut self, n: &u32) -> bool {
        if *n == 2 {
            return true;
        }
        if *n < 2 || *n & 1 == 0 {
            return false;
        }
        let mut i = 2;
        while i * i <= *n {
            if *n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /** Test if `n` is a prime in $O(\sqrt{n})$ time. */
    fn is_prime_u64(&mut self, n: &u64) -> bool {
        if *n < u32::MAX as u64 {
            return self.is_prime_u32(&(*n as u32));
        }
        if *n < 2 || *n & 1 == 0 {
            return false;
        }
        let mut i = 2;
        while i * i <= *n {
            if *n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl ExtractPrimeFactors for BruteForcePrimalityTest {
    /** Extract all prime factors from an 32-bit integer. */
    fn extract_prime_factors_u32(&mut self, n: &u32) -> Vec<u32> {
        let mut ans = Vec::new();
        let mut n = *n;
        while n & 1 == 0 {
            ans.push(2);
            n >>= 1;
        }
        let mut i = 2;
        while i * i <= n {
            while n % i == 0 {
                ans.push(i);
                n /= i;
            } 
            i += 1;
        }
        if n != 1 {
            ans.push(n);
        }
        ans
    }

    /** Extract all prime factors from an 64-bit integer. */
    fn extract_prime_factors_u64(&mut self, n: &u64) -> Vec<u64> {
        if *n <= u32::MAX as u64 {
            return self.extract_prime_factors_u32(&(*n as u32)).iter().map(|x| *x as u64).collect();
        }
        let mut ans = Vec::new();
        let mut n = *n;
        while n & 1 == 0 {
            ans.push(2);
            n >>= 1;
        }
        let mut i = 2;
        while i * i <= n {
            while n % i == 0 {
                ans.push(i);
                n /= i;
            } 
            i += 1;
        }
        if n != 1 {
            ans.push(n);
        }
        ans
    }

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from 32-bit `n`. */
    fn extract_factor_u32(&mut self, n: &u32) -> Option<u32> {
        if *n <= 2 {
            return None;
        }
        if *n & 1 == 0 {
            return Some(2);
        }
        let mut i = 2;
        while i * i <= *n {
            if n % i == 0 {
                return Some(i);
            } 
            i += 1;
        }
        None
    }

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from 64-bit `n`. */
    fn extract_factor_u64(&mut self, n: &u64) -> Option<u64> {
        if *n <= u32::MAX as u64 {
            return self.extract_factor_u32(&(*n as u32)).map(|x| x as u64);
        }
        if *n & 1 == 0 {
            return Some(2);
        }
        let mut i = 2;
        while i * i <= *n {
            if *n % i == 0 {
                return Some(i);
            } 
            i += 1;
        }
        None
    }

}

/** `MillerRabin` is an efficient algorithm of primality test. Its computation complexity is $O(k\log^2 n)$ where $k$ is the number of rounds performed and $n$ is the number tested for primality. Its accuracy is $4^{-k}$. */
#[derive(Clone, Debug)]
pub struct MillerRabin<RNG: Pseudorandom64> {
    rng: IntGenerator<RNG>,
    tests: u32,
}

impl<RNG: Pseudorandom64> MillerRabin<RNG> {
    /** New a `MillerRabin` with a given pseudorandom number generator `rng` and the number of rounds `tests`. If `tests` is `None` then set `20 `as the number of tests. */
    pub fn new(rng: &RNG, tests: Option<u32>) -> MillerRabin<RNG> {
        MillerRabin {
            rng: IntGenerator::new(rng),
            tests: if tests.is_none() { 20 } else { tests.unwrap() },
        }
    }
}

impl<RNG: Pseudorandom64> PrimalityTest for MillerRabin<RNG> {
    /** Test if 32-bit `n` is a prime. */
    fn is_prime_u32(&mut self, n: &u32) -> bool {
        let mut expected_step:u64 = (((32 - n.leading_zeros()) * (32 - n.leading_zeros())) as u64) * (self.tests as u64); 
        if (*n as u64) <= expected_step * expected_step {
            return BruteForcePrimalityTest::new().is_prime_u32(n);
        }
        let mut d = *n - 1;
        let n_1 = (*n-1) as u64; 
        let nu64 = *n as u64;
        let mut s = 0;
        while d & 1 == 0 {
            d >>= 1;
            s += 1;
        }
        for _ in 0..self.tests {
            let mut a = self.rng.gen_range_u32(2..*n);
            let mut x = pow_mod_u32(&a, &d, &n) as u64;
            if x == 1 || x == n_1 {
                continue;
            }
            let mut i = 0;
            while i < s {
                if x == n_1 {
                    break;
                }
                i += 1;
                x = x * x % nu64;
            }
            if i == s {
                return false;
            }
        }
        true
    }

    /** Test if 64-bit `n` is a prime. */
    fn is_prime_u64(&mut self, n: &u64) -> bool {
        if *n <= u32::MAX as u64 {
            return self.is_prime_u32(&(*n as u32));
        }
        let mut d = *n - 1;
        let n_1 = (*n-1) as u128; 
        let nu128 = *n as u128;
        let mut s = 0;
        while d & 1 == 0 {
            d >>= 1;
            s += 1;
        }
        for _ in 0..self.tests {
            let mut a = self.rng.gen_range_u64(2..*n);
            let mut x = pow_mod_u64(&a, &d, &n) as u128;
            if x == 1 || x == n_1 {
                continue;
            }
            let mut i = 0;
            while i < s {
                if x == n_1 {
                    break;
                }
                i += 1;
                x = x * x % nu128;
            }
            if i == s {
                return false;
            }
        }
        true
    }
}


/** `PollardRho` is an algorithm for integer factorization. */
#[derive(Clone, Debug)]
pub struct PollardRho<RNG: Pseudorandom64, PT: PrimalityTest> {
    rng: IntGenerator<RNG>,
    primality_tester: PT, 
}

impl<RNG: Pseudorandom64, PT: PrimalityTest> PollardRho<RNG, PT> {
    /** New a `PollardRho` with a given pseudorandom 64-bit number generator `rng` and a given primality tester `primality_tester`. */
    pub fn new(rng: &RNG, primality_tester: &PT) -> PollardRho<RNG, PT> {
        PollardRho {
            rng: IntGenerator::new(rng),
            primality_tester: primality_tester.clone(),
        }
    }
}

impl<RNG: Pseudorandom64, PT: PrimalityTest> ExtractPrimeFactors for PollardRho<RNG, PT> {
    /** Extract all prime factors from an 32-bit integer. */
    fn extract_prime_factors_u32(&mut self, n: &u32) -> Vec<u32> {
        let mut ans = Vec::new();
        let mut n = *n;
        while n & 1 == 0 {
            ans.push(2);
            n >>= 1;
        }
        if n == 1 {
            return ans;
        }
        let mut d = vec![n];
        while d.len() > 0 {
            let mut x = d.pop().unwrap();
            match self.extract_factor_u32(&x) {
                Some(p) => {
                    let ret = self.extract_prime_factors_u32(&p);
                    ans.append(&mut ret.clone());
                    x /= p;
                    for i in ret {
                        while x % i == 0 {
                            ans.push(i);
                            x /= i;
                        }
                        if x == 1 {
                            break;
                        }
                    }
                    if x != 1 {
                        d.push(x);
                    }
                },
                None => {
                    ans.push(x);
                },
            }
        }
        ans
    }

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from 32-bit `n`. If no such a factor then return `None`. It takes $O(n^{0.25})$ time.  */
    fn extract_factor_u32(&mut self, n: &u32) -> Option<u32> {
        if self.primality_tester.is_prime_u32(n) {
            return None;
        }
        if *n <= 2 {
            return None;
        }
        if *n & 1 == 0 {
            return Some(2);
        }
        let mut s = 0u64;
        let mut t = 0u64;
        let c = self.rng.gen_range_u32(1..*n) as u64;
        let n64 = *n as u64;
        let mut goal = 1u64;
        loop {
            let mut val = 1u64;
            for i in 1..=goal {
                t = (t * t) %n64 + c;
                if t >= n64 {
                    t -= n64;
                }
                if t > s {
                    val = val * (t - s) % n64;
                } else {
                    val = val * (s - t) % n64;
                }
                if val == 0 {
                    if t != s {
                        if t > s {
                            return Some(gcd_u32((t - s) as u32, *n));
                        } else {
                            return Some(gcd_u32((s - t) as u32, *n));
                        }
                    }
                    return self.extract_factor_u32(n);
                }
                if i % 127 == 0 {
                    let d = gcd_u32(val as u32, *n);
                    if d > 1 {
                        return Some(d);
                    }
                }
            }    
            let d = gcd_u32(val as u32, *n);
            if d > 1 {
                return Some(d);
            }
            s = t.clone();
            goal <<= 1;
        }
    }

    /** Extract all prime factors from an 64-bit integer. */
    fn extract_prime_factors_u64(&mut self, n: &u64) -> Vec<u64> {
        if *n <= u32::MAX as u64 {
            return self.extract_prime_factors_u32(&(*n as u32)).iter().map(|x| *x as u64).collect();
        }
        let mut ans = Vec::new();
        let mut n = *n;
        while n & 1 == 0 {
            ans.push(2);
            n >>= 1;
        }
        if n == 1 {
            return ans;
        }
        let mut d = vec![n];
        while d.len() > 0 {
            let mut x = d.pop().unwrap();
            match self.extract_factor_u64(&x) {
                Some(p) => {
                    let ret = self.extract_prime_factors_u64(&p);
                    ans.append(&mut ret.clone());
                    x /= p;
                    for i in ret {
                        while x % i == 0 {
                            ans.push(i);
                            x /= i;
                        }
                        if x == 1 {
                            break;
                        }
                    }
                    if x != 1 {
                        d.push(x);
                    }
                },
                None => {
                    ans.push(x);
                },
            }
        }
        ans
    }

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from 64-bit `n`. If no such a factor then return `None`. It takes $O(n^{0.25})$ time.  */
    fn extract_factor_u64(&mut self, n: &u64) -> Option<u64> {
        if *n <= u32::MAX as u64 {
            return self.extract_factor_u32(&(*n as u32)).map(|x| x as u64);
        }
        if self.primality_tester.is_prime_u64(n) {
            return None;
        }
        if *n & 1 == 0 {
            return Some(2);
        }
        let mut s = 0u128;
        let mut t = 0u128;
        let c = self.rng.gen_range_u64(1..*n) as u128;
        let n128 = *n as u128;
        let mut goal = 1u64;
        loop {
            let mut val = 1u128;
            for i in 1..=goal {
                t = (t * t) %n128 + c;
                if t >= n128 {
                    t -= n128;
                }
                if t > s {
                    val = val * (t - s) % n128;
                } else {
                    val = val * (s - t) % n128;
                }
                if val == 0 {
                    if t != s {
                        if t > s {
                            return Some(gcd_u64((t - s) as u64, *n));
                        } else {
                            return Some(gcd_u64((s - t) as u64, *n));
                        }
                    }
                    return self.extract_factor_u64(n);
                }
                if i % 127 == 0 {
                    let d = gcd_u64(val as u64, *n);
                    if d > 1 {
                        return Some(d);
                    }
                }
            }    
            let d = gcd_u64(val as u64, *n);
            if d > 1 {
                return Some(d);
            }
            s = t.clone();
            goal <<= 1;
        }
    }
}