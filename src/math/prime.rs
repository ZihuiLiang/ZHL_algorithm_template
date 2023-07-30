use crate::{random::generator::{Pseudorandom64, IntGenerator}, math::basic::{pow_mod, gcd}};

/** `PrimalityTest` introduces a trait for primality test. */
pub trait PrimalityTest: Clone {
    /** Test if $n$ is a prime. Note that `T` should be able to contain $n^2$. */
    fn is_prime<T>(&mut self, n: &T) -> bool where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T>  + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug;
}

/** `FindPrimeFactors` introduces a trait for Extracting prime factors from an integer.*/
pub trait ExtractPrimeFactors: Clone {
    /** Extract prime factors from an integer. Note that `T` should be able to contain $n^2$. */
    fn extract_prime_factors<T>(&mut self, n: &T) -> Vec<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T> + std::ops::DivAssign + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug;

    /** Extract a (not necessarily prime) factor from an integer. Note that `T` should be able to contain $n^2$. */
    fn extract_factor<T>(&mut self, n: &T) -> Option<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T>  + std::ops::Div<Output = T> + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug;

}

/** `BruteForcePrimalityTest` is a naive algorithm of primality test. It's computation complexity is $O(\sqrt{n})$. */
#[derive(Clone, Debug)]
pub struct BruteForcePrimalityTest {}

impl BruteForcePrimalityTest {
    /** New a `BruteForcePrimalityTest`. */
    pub fn new() -> BruteForcePrimalityTest {
        BruteForcePrimalityTest {}
    }
}

impl PrimalityTest for BruteForcePrimalityTest {
    /** Test if $n$ is a prime in $O(\sqrt{n})$ time. */
    fn is_prime<T>(&mut self, n: &T) -> bool where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T>  + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug {
        let zero: T = 0.try_into().unwrap();
        let one: T = 1.try_into().unwrap();
        let two: T = 2.try_into().unwrap();
        if *n == two {
            return true;
        }
        if *n < two || *n & one == zero {
            return false;
        }
        assert_eq!(*n * *n / *n, *n); // check T can contain *n * *n
        let mut i = two;
        while i * i <= *n {
            if *n % i == zero {
                return false;
            }
            i += one;
        }
        true
    }
}

impl ExtractPrimeFactors for BruteForcePrimalityTest {
    /** Extract prime factors from an integer. */
    fn extract_prime_factors<T>(&mut self, n: &T) -> Vec<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T> + std::ops::DivAssign + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug {
        let mut ans = Vec::new();
        let mut n = *n;
        let zero: T = 0.try_into().unwrap();
        let one: T = 1.try_into().unwrap();
        let two: T = 2.try_into().unwrap();
        while n & one == zero {
            ans.push(two);
            n >>= 1;
        }
        let mut i = two;
        while i * i <= n {
            if n % i == zero {
                ans.push(i);
                n /= i;
            } else {
                i += one;
            }
        }
        if n != one {
            ans.push(n);
        }
        ans
    }

    /** Extract a factor from an integer. */
    fn extract_factor<T>(&mut self, n: &T) -> Option<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::AddAssign + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Rem<Output = T>  + std::ops::Div<Output = T> + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug {
        let mut n = *n;
        let zero: T = 0.try_into().unwrap();
        let one: T = 1.try_into().unwrap();
        let two: T = 2.try_into().unwrap();
        while n & one == zero {
            return Some(two);
        }
        let mut i = two;
        while i * i <= n {
            if n % i == zero {
                return Some(i);
            } else {
                i += one;
            }
        }
        None
    }
}

/** `MillerRabin` is an efficient algorithm of primality test. It's computation complexity is $O(k\log^2 n)$ where $k$ is the number of rounds performed and $n$ is the number tested for primality. It's accuracy is $4^{-k}$. */
#[derive(Clone, Debug)]
pub struct MillerRabin<RNG: Pseudorandom64> {
    rng: IntGenerator<RNG>,
    tests: usize,
}

impl<RNG: Pseudorandom64> MillerRabin<RNG> {
    /** New a `MillerRabin` with a given pseudorandom number generator `rng` and the number of rounds `tests`. If `tests` is `None` then set `20 `as the number of tests. */
    pub fn new(rng: &RNG, tests: Option<usize>) -> MillerRabin<RNG> {
        MillerRabin {
            rng: IntGenerator::new(rng),
            tests: if tests.is_none() { 20 } else { tests.unwrap() },
        }
    }
}

impl<RNG: Pseudorandom64> PrimalityTest for MillerRabin<RNG> {
    /** Test if $n$ is a prime. */
    fn is_prime<T>(&mut self, n: &T) -> bool where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T>  + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug{
        let zero: T = 0.try_into().unwrap();
        let one: T = 1.try_into().unwrap();
        let two: T = 2.try_into().unwrap();
        let mut expected_step:u64 = (std::mem::size_of::<T>() * 8 * self.tests).try_into().unwrap(); 
        expected_step = expected_step * expected_step;
        let bound: T = expected_step.try_into().unwrap();
        if n < &bound {
            return BruteForcePrimalityTest::new().is_prime(n);
        }
        assert_eq!(*n * *n / *n, *n); // check T can contain *n * *n
        let mut d = *n - one;
        let mut s = 0;
        while d & one == zero {
            d >>= 1;
            s += 1;
        }
        for _ in 0..self.tests {
            let mut a = self.rng.gen_range::<T>(two..*n);
            let mut x = pow_mod(a, d, n);
            if x == one || x == *n - one {
                continue;
            }
            let mut i = 0;
            while i < s {
                if x == *n - one {
                    break;
                }
                i += 1;
                x = x * x % *n;
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
pub struct PollardRho<RNG: Pseudorandom64, PD: PrimalityTest> {
    rng: IntGenerator<RNG>,
    prime_detector: PD, 
}

impl<RNG: Pseudorandom64, PD: PrimalityTest> PollardRho<RNG, PD> {
    /** New a `PollardRho` with a given pseudorandom 64-bit number generator `rng`. */
    pub fn new(rng: &RNG, prime_detector: &PD) -> PollardRho<RNG, PD> {
        PollardRho {
            rng: IntGenerator::new(rng),
            prime_detector: prime_detector.clone(),
        }
    }
}

impl<RNG: Pseudorandom64, PD: PrimalityTest> ExtractPrimeFactors for PollardRho<RNG, PD> {
    /** Extract prime factors from an integer. */
    fn extract_prime_factors<T>(&mut self, n: &T) -> Vec<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::AddAssign + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T> + std::ops::DivAssign + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug {
        let mut ans = Vec::new();
        let mut n = *n;
        let zero: T = 0.try_into().unwrap();
        let one: T = 1.try_into().unwrap();
        let two: T = 2.try_into().unwrap();
        while n & one == zero {
            ans.push(two);
            n >>= 1;
        }
        let mut d = vec![n];
        while d.len() > 0 {
            let mut x = d.pop().unwrap();
            match self.extract_factor(&x) {
                Some(p) => {
                    let ret = self.extract_prime_factors(&p);
                    ans.append(&mut ret.clone());
                    x /= p;
                    for i in ret {
                        while x % i == zero {
                            ans.push(i);
                            x /= i;
                        }
                        if x == one {
                            break;
                        }
                    }
                    if x != one {
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

    /** Extract a prime factor from an integer. $O(n^{0.25})$ */
    fn extract_factor<T>(&mut self, n: &T) -> Option<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::AddAssign + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Rem<Output = T>  + std::ops::Div<Output = T> + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug {
        if self.prime_detector.is_prime(n) {
            return None;
        }
        let zero: T = 0.try_into().unwrap();
        let one: T = 1.try_into().unwrap();
        let two: T = 2.try_into().unwrap();
        if *n == two {
            return None;
        }
        if *n < two || *n & one == zero {
            return None;
        }
        assert_eq!(*n * *n / *n, *n); // check T can contain *n * *n
        let mut s = zero.clone();
        let mut t = zero.clone();
        let c = self.rng.gen_range::<T>(one..*n);
        let mut goal = 1u64;
        loop {
            let mut val = one.clone();
            for i in 1..=goal {
                t = (t * t + c) % *n;
                if t > s {
                    val = val * (t - s) % *n;
                } else {
                    val = val * (s - t) % *n;
                }
                if val == zero {
                    if t != s {
                        if t > s {
                            return Some(gcd(t - s, *n));
                        } else {
                            return Some(gcd(s - t, *n));
                        }
                    }
                    return self.extract_factor(n);
                }
                if i % 127 == 0 {
                    let d = gcd(val, *n);
                    if d > one {
                        return Some(d);
                    }
                }
            }    
            let d = gcd(val, *n);
            if d > one {
                return Some(d);
            }
            s = t.clone();
            goal <<= 1;
        }
    }
}