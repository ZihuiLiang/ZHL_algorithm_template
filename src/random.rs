/*! This crate demonstrates the random algorithms and datastructures.
*/
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
pub struct IntGenerator<RNG: Pseudorandom64>{
    rng: RNG,
}


impl<RNG: Pseudorandom64> IntGenerator<RNG> {

    /** New a `NumGenerator` with a given pseudorandom number generator `rng`. */
    pub fn new(rng: &RNG) -> IntGenerator<RNG> {
        IntGenerator {
            rng: rng.clone()
        }
    }

    /** Generate a `T`-type integer. */
    pub fn gen<T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::Shl<i32, Output = T> + Copy>(&mut self) -> T where <T as TryFrom<u64>>::Error: std::fmt::Debug {
        match std::mem::size_of::<T>() {
            1 => {
                let v = self.rng.gen();
                let x: T = (v & 0xf).try_into().unwrap();
                let y: T = ((v >> 4) & 0xf).try_into().unwrap();
                x << 4 | y
            }, 
            2 => {
                let v = self.rng.gen();
                let x: T = (v & 0xff).try_into().unwrap();
                let y: T = ((v >> 8) & 0xff).try_into().unwrap();
                x << 8 | y
            },
            4 => {
                let v = self.rng.gen();
                let x: T = (v & 0xffff).try_into().unwrap();
                let y: T = ((v >> 16) & 0xffff).try_into().unwrap();
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
    pub fn gen_range<T: TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::Shl<i32, Output = T> + std::ops::Sub<Output = T> + std::ops::Rem<Output = T>  + std::ops::Add<Output = T> + Copy>(&mut self, range: Range<T>) -> T where <T as TryFrom<u64>>::Error: std::fmt::Debug {
        self.gen::<T>() % (range.end - range.start) + range.start
    }

    /** Random `k` integers from `range`. $O(k)$ time with `can_repeat=true`. Expected $O(k)$ time with `can_repeat=false`. */
    pub fn gen_range_k
        <T:
        TryFrom<u64> + std::ops::BitOr<Output = T> + std::ops::Shl<i32, Output = T> + std::ops::Sub<Output = T> + std::ops::Rem<Output = T>  + std::ops::Add<Output = T> + Copy +// for gen_range
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
#[derive(Clone)]
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