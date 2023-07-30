/** Problem link: https://www.luogu.com.cn/problem/P4718 */
use std::io::stdin;
fn main() {
    let mut rng = MT19937_64::new(None);
    let mut MillerRabin = MillerRabin::new(&mut rng, Some(8));
    let mut rng = MT19937_64::new(None);
    let mut pollard_rho = PollardRho::new(&mut rng, &MillerRabin);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let n: u128 = input.trim().parse().unwrap();
        let primes = pollard_rho.extract_prime_factors(&n);
        if primes.len() == 1 {
            println!("Prime");
        } else {
            println!("{}", primes.iter().max().unwrap());
        }
    }
}

/** `pow_mod` is an algorithm to compute $a^b \mod c$ in $O(\log(b)\cdot \log^2(c))$ where $O(\log^2(c))$ is the complexity of big integer operations. Note that $T$ should be able to contain $c^2$. */
pub fn pow_mod<T: TryFrom<u64> + std::ops::ShrAssign<usize>>(mut a: T, mut b: T, c: &T) -> T where T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T> + std::ops::BitAnd<Output = T> + std::cmp::PartialEq + std::cmp::PartialOrd + Copy + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug{
    let one: T = 1.try_into().unwrap();
    let zero: T = 0.try_into().unwrap();
    let mut ans: T = one.clone();
    assert_eq!(*c * *c / *c, *c);
    while b > zero {
        if b & one == one {
            ans = ans * a % *c;
        }
        a = a * a % *c;
        b >>= 1;
    }
    ans
}

/** `gcd` is an algorithm to compute the greatest common divisor of two integers $a$ and $b$ in $O(\log^3(a+b))$. */
pub fn gcd<T: TryFrom<u64>>(mut a: T, mut b: T) -> T where T: std::ops::Rem<Output = T> + std::cmp::PartialEq + std::cmp::PartialOrd + Copy, <T as TryFrom<u64>>::Error: std::fmt::Debug {
    let zero: T = 0.try_into().unwrap();
    while b > zero {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

use std::{collections::HashSet, ops::Range, time::SystemTime};


/** `Pseudorandom64` introduces a trait for pseudorandom 64-bit unsigned integers. */
pub trait Pseudorandom64: Clone {
    /** New a with a given seed. */
    fn new(seed: Option<u64>) -> Self;
    
    /** Set the seed. */
    fn seed(&mut self, seed: u64);
    
    /** Generate a 64-bit unsigned integer. */
    fn gen(&mut self) -> u64;
}

/** `IntGenerator` is a generator used for generating integers. */
#[derive(Clone, Debug)]
pub struct IntGenerator<RNG: Pseudorandom64>{
    rng: RNG,
    v_1: u64,
    cnt_1: u8,
    v_2: u64,
    cnt_2: u8,
    v_4: u64,
    cnt_4: u8,
}


impl<RNG: Pseudorandom64> IntGenerator<RNG> {

    /** New a `NumGenerator` with a given pseudorandom number generator `rng`. */
    pub fn new(rng: &RNG) -> IntGenerator<RNG> {
        IntGenerator {
            rng: rng.clone(),
            v_1: 0u64,
            cnt_1: 8,
            v_2: 0u64,
            cnt_2: 8,
            v_4: 0u64,
            cnt_4: 8,
        }
    }

    /** Generate a `T`-type integer. */
    pub fn gen<T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::Shl<usize, Output = T> + Copy>(&mut self) -> T where <T as TryFrom<u64>>::Error: std::fmt::Debug {

        match std::mem::size_of::<T>() {
            1 => {
                if self.cnt_1 == 8 {
                    self.v_1 = self.rng.gen();
                    self.cnt_1 = 0;
                }
                let x: T = (self.v_1 & 0xf).try_into().unwrap();
                let y: T = ((self.v_1 >> 4) & 0xf).try_into().unwrap();
                self.cnt_1 += 1;
                self.v_1 >>= 8;
                x << 4 | y
            }, 
            2 => {
                if self.cnt_2 == 8 {
                    self.v_2 = self.rng.gen();
                    self.cnt_2 = 0;
                }
                let x: T = (self.v_2 & 0xff).try_into().unwrap();
                let y: T = ((self.v_2 >> 8) & 0xff).try_into().unwrap();
                self.cnt_2 += 2;
                self.v_2 >>= 16;
                x << 8 | y
            },
            4 => {
                if self.cnt_4 == 8 {
                    self.v_4 = self.rng.gen();
                    self.cnt_4 = 0;
                }
                let x: T = (self.v_4 & 0xffff).try_into().unwrap();
                let y: T = ((self.v_4 >> 16) & 0xffff).try_into().unwrap();
                self.cnt_4 += 4;
                self.v_4 >>= 32;
                x << 16 | y
            },
            8 => {
                let v = self.rng.gen();
                let x: T = (v & 0xffffffff).try_into().unwrap();
                let y: T = ((v >> 32) & 0xffffffff).try_into().unwrap();
                x << 32 | y
            },
            16 => {
                let x: T = self.rng.gen().try_into().unwrap();
                let y: T = self.rng.gen().try_into().unwrap();
                x << 64 | y
            },
            _ => panic!("Unsupported type."),
        }
    }

    /** Generate an integer in `range` randomly. */
    pub fn gen_range<T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::Rem<Output = T>  + std::ops::Add<Output = T> + Copy>(&mut self, range: Range<T>) -> T where <T as TryFrom<u64>>::Error: std::fmt::Debug {
        self.gen::<T>() % (range.end - range.start) + range.start
    }

    /** Random `k` integers from `range`. $O(k)$ time with `can_repeat=true`. Expected $O(k)$ time with `can_repeat=false`. */
    pub fn gen_range_k
        <T:
        TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::Rem<Output = T>  + std::ops::Add<Output = T> + Copy +// for gen_range
        TryInto<u128> + std::hash::Hash + Copy + std::cmp::PartialOrd + std::cmp::Eq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Sized
        >
        (&mut self, range: Range<T>, k: usize, can_repeat: bool) -> Vec<T> 
        where 
        <T as TryFrom<u64>>::Error: std::fmt::Debug, //for gen_range_k
        std::ops::Range<T>: Iterator, std::ops::RangeInclusive<T>: Iterator<Item = T>, <T as TryInto<u128>>::Error: std::fmt::Debug 
        { 
        if range.start > range.end {
            panic!("The range is invalid.");
        }
        
        if can_repeat {
            let mut ret = Vec::new();
            for _ in 0..k {
                ret.push(self.gen_range(range.clone()));
            }
            return ret;
        } 

        let range_len: u128 = {
            if range.start >= 0.try_into().unwrap() {
                let start: u128 = range.start.try_into().unwrap();
                let end: u128 = range.end.try_into().unwrap();
                end - start
            } else {
                let zero: T = 0.try_into().unwrap();
                let start: u128 = (zero - range.start).try_into().unwrap();
                let end: u128 = range.end.try_into().unwrap();
                end + start
            }
        };

        if range_len < k as u128 {
            panic!("The range is too small to generate {} numbers.", k);
        }

        if range_len > (k as u128) * 5 {
            let mut HashSet = std::collections::HashSet::new();
            while HashSet.len() < k {
                HashSet.insert(self.gen_range(range.clone()));
            }
            return HashSet.into_iter().collect::<Vec<T>>();
        }

        let mut x = range.start.clone();
        let mut ret = vec![];
        let one: T = 1.try_into().unwrap();
        while x < range.end {
            ret.push(x.clone());
            x = x + one;
        }
        self.random_shuffle(&mut ret);
        ret[0..k].to_vec()
    }

    /** Generate an element from vector `d` randomly. */
    pub fn gen_from_vec<T: Clone>(&mut self, d: &Vec<T>) -> T {
        let ret = self.gen_range::<usize>(0..d.len());
        d[ret].clone()
    }

    /** Generate `k` elements from vector `d`. If `can_repeat=true` then generating repeated elements is allowed, otherwise all generated elements are distinct. */
    pub fn gen_from_vec_k<T: Clone>(&mut self, d: &Vec<T>, k: usize, can_repeat: bool) -> Vec<T> {
        self.gen_range_k::<usize>(0..d.len(), k, can_repeat).iter().map(|&x| d[x].clone()).collect::<Vec<T>>()
    }

    /** Random shuffle a slice. */
    pub fn random_shuffle<T: Copy>(&mut self, slice: &mut [T]) {
        let mut i = slice.len();
        while i > 1 {
            let j = self.gen_range(0..i);
            i -= 1;
            slice.swap(i, j);
        }
    }

    /** Split a vector `d` into `k` parts. If `can_empty=true` then the generating empty splited parts is allowed, otherwise all splited parts are non-empty. */
    pub fn split_vec_k<T: Clone>(&mut self, d: &Vec<T>, k: usize, can_empty: bool) -> Vec<Vec<T> > {
        if can_empty == false && d.len() < k {
            panic!("The length of the vector is too small to split into {} parts.", k);
        }
        let mut range = if can_empty {0..d.len() + 1} else {1..d.len()};
        let mut ret = Vec::new();
        let mut q = self.gen_range_k(range, k - 1, can_empty);
        q.sort();
        let mut start = 0;
        for i in q {
            ret.push(d[start..i].to_vec());
            start = i;
        }
        ret.push(d[start..].to_vec());
        ret
    }
}


/** `MT19937_64` is a general-purpose pseudorandom number generator. The core of this structure is to generate 64-bit integers. */
#[derive(Clone, Debug)]
pub struct MT19937_64 {
    mt: [u64; 312],
    index: usize,
}

impl MT19937_64 {
    const LOWER_MASK: u64 = (1 << 31) - 1;
    const UPPER_MASK: u64 = !((1 << 31) - 1);
    const MASK: u128 = (1 << 64) - 1;

    fn twist(&mut self) {
        for i in 0..312 {
            let x = (self.mt[i] & Self::UPPER_MASK) + (self.mt[(i + 1) % 312] & Self::LOWER_MASK);
            let mut x_a = x >> 1;
            if x % 2 != 0 {
                x_a = x_a ^ 0xB5026F5AA96619E9;
            }
            self.mt[i] = self.mt[(i + 156) % 312] ^ x_a;
        }
        self.index = 0;
    }
}

impl Pseudorandom64 for MT19937_64 {

    /** New a `MT19937_64` with a `seed`. If `seed` is `None`, a seed selected by `SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros()` is applied. */
    fn new(seed: Option<u64>) -> Self {
        let mut rng = MT19937_64 {
            mt: [0; 312],
            index: 0,
        };
        match seed {
            Some(seed) => rng.seed(seed),
            None => rng.seed(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros() as u64),
        }
        rng
    }

    /** Initialize `MT19937_64` with a `seed`. */
    fn seed(&mut self, seed: u64) {
        self.index = 312;
        self.mt[0] = seed;
        for i in 1..312 {
            self.mt[i] = ((6364136223846793005 as u128 * (self.mt[i - 1] ^ (self.mt[i - 1] >> 62)) as u128 + i as u128) & Self::MASK)as u64;
        }
    }

    /** Generate a 64-bit integer. */
    fn gen(&mut self) -> u64 {
        if self.index >= 312 {
            self.twist();
        }
        let mut y = self.mt[self.index];
        y = y ^ ((y >> 29) & 0x5555555555555555);
        y = y ^ ((y << 17) & 0x71D67FFFEDA60000);
        y = y ^ ((y << 37) & 0xFFF7EEE000000000);
        y = y ^ (y >> 43);
        self.index += 1;
        y
    }
}


/** `PrimalityTest` introduces a trait for primality test. */
pub trait PrimalityTest: Clone {
    /** Test if `n` is a prime. Note that `T` should be able to contain $n^2$. */
    fn is_prime<T>(&mut self, n: &T) -> bool where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T>  + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug;
}

/** `FindPrimeFactors` introduces a trait for Extracting prime factors from an integer.*/
pub trait ExtractPrimeFactors: Clone {
    /** Extract all prime factors from an integer. Note that `T` should be able to contain $n^2$. */
    fn extract_prime_factors<T>(&mut self, n: &T) -> Vec<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + std::ops::Div<Output = T> + std::ops::DivAssign + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug;

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from `n`. Note that `T` should be able to contain $n^2$. */
    fn extract_factor<T>(&mut self, n: &T) -> Option<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + std::ops::Rem<Output = T>  + std::ops::Div<Output = T> + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug;

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
        let mut i2: T = 4.try_into().unwrap(); 
        while i2 <= *n {
            if *n % i == zero {
                return false;
            }
            i2 += one + (i << 1);
            i += one;
        }
        true
    }
}

impl ExtractPrimeFactors for BruteForcePrimalityTest {
    /** Extract all prime factors from an integer. */
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
        let mut i2: T = 4.try_into().unwrap();
        while i2 <= n {
            while n % i == zero {
                ans.push(i);
                n /= i;
            } 
            i2 += one + (i << 1);
            i += one;
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
        let mut i2: T = 4.try_into().unwrap();
        while i2 <= n {
            if n % i == zero {
                return Some(i);
            } else {
                i2 += one + (i << 1);
                i += one;
            }
        }
        None
    }
}

/** `MillerRabin` is an efficient algorithm of primality test. Its computation complexity is $O(k\log^2 n)$ where $k$ is the number of rounds performed and $n$ is the number tested for primality. Its accuracy is $4^{-k}$. */
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
    /** Test if `n` is a prime. */
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
    /** Extract all prime factors from an integer. */
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

    /** Extract a (not necessarily prime) factor (which is strictly smaller than `n`) from `n`. If no such a factor then return `None`. It takes $O(n^{0.25})$ time.  */
    fn extract_factor<T>(&mut self, n: &T) -> Option<T> where T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::ShrAssign<usize> + std::ops::Shl<usize, Output = T> + std::ops::AddAssign + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Rem<Output = T>  + std::ops::Div<Output = T> + std::ops::Add<Output = T> + Copy + std::cmp::PartialEq + std::cmp::PartialOrd + std::ops::BitAnd<Output = T> + std::fmt::Debug, <T as TryFrom<u64>>::Error: std::fmt::Debug {
        if self.primality_tester.is_prime(n) {
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