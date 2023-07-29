use std::cmp;

/** `ZeroOneLinearEquation` is a linear equation in $\mathbf{Z}^{n}_2$. */
#[derive(Clone, Debug)]
pub struct ZeroOneLinearEquation {
    f: Vec<u128>,
    equation_size: usize,
}

impl ZeroOneLinearEquation {
    
    /** New a `ZeroOneLinearEquation` with `size` of dimensions. */
    pub fn new(size: usize) -> Self {
        ZeroOneLinearEquation {
            f: vec![0u128; (size + 127) / 128],
            equation_size: size,
        }
    }

    /** Set the `i`th bit to `1`. */
    pub fn set_i(&mut self, i: usize) {
        assert!(i < self.equation_size);
        self.f[i >> 7] |= 1 << (i & 127);
    }

    /** Set the `i`th bit to `0`. */
    pub fn unset_i(&mut self, i: usize) {
        assert!(i < self.equation_size);
        self.f[i >> 7] &= !(1 << (i & 127));
    }

    /** Flip the `i`th bit. */
    pub fn flip_i(&mut self, i: usize) {
        assert!(i < self.equation_size);
        self.f[i >> 7] ^= 1 << (i & 127);
    }

    /** Get the `i`th bit. */
    pub fn get_i(&self, i: usize) -> bool {
        assert!(i < self.equation_size);
        (self.f[i >> 7] >> (i & 127)) & 1 == 1
    }

    /** Count the number of 1 bits. */
    pub fn count_ones(&self) -> usize {
        self.f.iter().map(|x| x.count_ones() as usize).sum()
    }

    /** Count the number of 0 bits. */
    pub fn count_zeros(&self) -> usize {
        self.equation_size - self.count_ones()
    }

    /** Return whether the equation is all 1 bits. */
    pub fn is_all_ones(&self) -> bool {
        self.count_zeros() == 0
    }

    /** Return whether the equation is all 0 bits. */
    pub fn is_all_zeros(&self) -> bool {
        self.count_ones() == 0
    }

    /** Get the highest bit that is `1`. */
    pub fn highest_one_bit(&self) -> Option<usize> {
        for i in (0..self.f.len()).rev() {
            if self.f[i] != 0 {
                return Some(i * 128 + 127 - self.f[i].leading_zeros() as usize);
            }
        }
        return None;
    }

    /** Xor the equation with `other`. */
    pub fn xor(&mut self, other: &ZeroOneLinearEquation) {
        assert!(self.equation_size == other.equation_size);
        for i in 0..self.f.len() {
            self.f[i] ^= other.f[i];
        }
    }

    /** Return the size of the equation. */
    pub fn equation_size(&self) -> usize {
        self.equation_size
    }
}

/** Implement `PartialEq` for `ZeroOneLinearEquation`*/
impl PartialEq for ZeroOneLinearEquation {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.equation_size == other.equation_size);
        for i in 0..self.f.len() {
            if self.f[i] != other.f[i] {
                return false;
            }
        }
        true
    }
}

/** Implement `PartialOrd` for `ZeroOneLinearEquation`. a<b if the highest different bit in b is 1. */
impl PartialOrd for ZeroOneLinearEquation {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        assert!(self.equation_size == other.equation_size);
        for i in (0..self.f.len()).rev() {
            if self.f[i] > other.f[i] {
                return Some(cmp::Ordering::Greater);
            } else if self.f[i] < other.f[i] {
                return Some(cmp::Ordering::Less);
            }
        }
        Some(cmp::Ordering::Equal)
    }
}

/** `XorLinearEquationSystem` is a system of linear equations, ($\mathbf{Z}^{n}_2$, `Xor`). */
#[derive(Clone, Debug)]
pub struct XorLinearEquationSystem {
    equations: Vec<ZeroOneLinearEquation>,  // basis
    equation_size: usize,                   // the size of dimension
    highest_one_bit: Vec<usize>,                // the highest 1-bit of each equation in the basis
}

impl XorLinearEquationSystem {

    /** New a `XorLinearEquationSystem` with `equation_num` `ZeroOneLinearEquation`s and `equation_size` of dimensions. */
    pub fn new(equation_num: usize, equation_size: usize) -> Self {
        XorLinearEquationSystem {
            equations: vec![ZeroOneLinearEquation::new(equation_size); equation_num],
            equation_size: equation_size,
            highest_one_bit: vec![],
        }
    }

    /** New a `ZeroOneLinearEquation` with `equation_size` of dimensions. */
    pub fn new_equation(&mut self) -> ZeroOneLinearEquation {
        ZeroOneLinearEquation::new(self.equation_size)
    }

    /** Set the `j`th bit of the `i`th equation to `1`. */
    pub fn set_i_j(&mut self, i: usize, j: usize) {
        assert!(i < self.equations.len());
        self.equations[i].set_i(j);
    }

    /** Set the `j`th bit of the `i`th equation to `0`. */
    pub fn unset_i_j(&mut self, i: usize, j: usize) {
        assert!(i < self.equations.len());
        self.equations[i].unset_i(j);
    }

    /** Flip the `j`th bit of the `i`th equation. */
    pub fn flip_i_j(&mut self, i: usize, j: usize) {
        assert!(i < self.equations.len());
        self.equations[i].flip_i(j);
    }

    /** Count the number of 1 bits in `i`th equation. */
    pub fn count_ones_i(&self, i: usize) -> usize {
        assert!(i < self.equations.len());
        self.equations[i].count_ones()
    }

    /** Count the number of 0 bits in `i`th equation. */
    pub fn count_zeros_i(&self, i: usize) -> usize {
        assert!(i < self.equations.len());
        self.equation_size - self.count_ones_i(i)
    }

    /** Return whether the `i`th equation is all 1 bits. */
    pub fn is_all_ones_i(&self, i: usize) -> bool {
        self.count_zeros_i(i) == 0
    }

    /** Return whether the `i`th equation is all 0 bits. */
    pub fn is_all_zeros_i(&self, i: usize) -> bool {
        self.count_ones_i(i) == 0
    }

    /** Get the `j`th bit of `i`th equation. */
    pub fn get_i_j(&self, i: usize, j: usize) -> bool {
        assert!(i < self.equations.len());
        self.equations[i].get_i(j)
    }

    /** Get the `i`th equation. */
    pub fn get_i(&self, i: usize) -> &ZeroOneLinearEquation {
        assert!(i < self.equations.len());
        &self.equations[i]
    }

    /** Get the highest 1-bit of `i`th equation. */
    pub fn highest_one_bit_i(&self, i: usize) -> usize {
        assert!(i < self.highest_one_bit.len());
        self.highest_one_bit[i]
    }

    /** Check if the basis is full. */
    pub fn is_full_basis(&self) -> bool {
        self.equations.len() == self.equation_size
    }

    /** Compute the basis. Note that for each pair of equation[i] and equation[j], the equation[j].get(highest_one_bit[i])=0 and equation[i].get(highest_one_bit[j])=0. */
    pub fn compute_basis(&mut self) {
        let mut l = 0usize;
        for i in (0..self.equation_size).rev() {
            let mut j = l;
            while j < self.equations.len() && !self.equations[j].get_i(i) {
                j += 1;
            }
            if j == self.equations.len() {
                continue;
            }
            self.highest_one_bit.push(i);

            self.equations.swap(l, j);

            let tmp = self.equations[l].clone();
            
            for k in 0..l {
                if self.equations[k].get_i(i) {
                    self.equations[k].xor(& tmp);
                }
            }

            for k in l+1..self.equations.len() {
                if self.equations[k].get_i(i) {
                    self.equations[k].xor(& tmp);
                }
            }


            l += 1;
        }
        self.equations.truncate(l);
    }

    /** Check if the `equation` can be represented by the system. */
    pub fn can_represent(&self, mut equation: ZeroOneLinearEquation) -> bool {
        assert!(self.equation_size == equation.equation_size);
        for i in 0..self.equations.len() {
            if equation.get_i(self.highest_one_bit[i]) {
                equation.xor(&self.equations[i]);
            }
        }
        equation.highest_one_bit().is_none()
    }

    /** Insert a new equation into the basis. If the basis is extended, return true, otherwise false. */
    pub fn insert_basis(&mut self, mut equation: ZeroOneLinearEquation) -> bool {
        assert!(self.equation_size == equation.equation_size);
        for i in 0..self.equations.len() {
            if equation.get_i(self.highest_one_bit[i]) {
                equation.xor(&self.equations[i]);
            }
            let highest_one_bit = equation.highest_one_bit();
            if highest_one_bit.is_none() {
                return false;   
            } 
            let highest_one_bit = highest_one_bit.unwrap();
            if highest_one_bit > self.highest_one_bit[i] {
                for j in i..self.equations.len() {
                    if equation.get_i(self.highest_one_bit[j]) {
                        equation.xor(&self.equations[j]);
                    }
                }
                for j in 0..i {
                    if self.equations[j].get_i(highest_one_bit) {
                        self.equations[j].xor(&equation);
                    }
                }
                self.equations.insert(i, equation);
                self.highest_one_bit.insert(i, highest_one_bit);
                return true;
            }
        }
        let highest_one_bit = equation.highest_one_bit();
        if highest_one_bit.is_none() {
            return false;   
        } 
        let highest_one_bit = highest_one_bit.unwrap();
        for i in 0..self.equations.len() {
            if self.equations[i].get_i(highest_one_bit) {
                self.equations[i].xor(&equation);
            }
        }
        self.equations.push(equation);
        self.highest_one_bit.push(highest_one_bit);
        true
    }

    /** Get the maximum equation that can be represented by the system. */
    pub fn get_max(&self) -> ZeroOneLinearEquation{
        let mut ans = self.equations[0].clone();
        for i in 1..self.equations.len() {
            if !ans.get_i(self.highest_one_bit[i]) {
                ans.xor(&self.equations[i]);
            }
        }
        return ans;
    }

    /** Return the size of the basis. */
    pub fn count_basis(&self) -> usize {
        self.equations.len()
    }

    /** Return the size of the equation. */
    pub fn equation_size(&self) -> usize {
        self.equation_size
    }
}