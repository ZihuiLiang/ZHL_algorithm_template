/*! This crate demonstrates the algorithms for solving math problems.
*/


/** `ZeroOneLinearEquation` is a linear equation in $\mathbf{Z}^{n}_2$. */
#[derive(Clone, Debug)]
pub struct ZeroOneLinearEquation {
    f: Vec<i128>
}

impl ZeroOneLinearEquation {
    
    /** New a `ZeroOneLinearEquation` with `size` of dimensions. */
    pub fn new(size: usize) -> Self {
        ZeroOneLinearEquation {
            f: vec![0i128; (size + 127) / 128],
        }
    }

    /** Set the `i`th bit to `1`. */
    pub fn set_i(&mut self, i: usize) {
        self.f[i >> 7] |= 1 << (i & 127);
    }

    /** Set the `i`th bit to `0`. */
    pub fn unset_i(&mut self, i: usize) {
        self.f[i >> 7] &= !(1 << (i & 127));
    }

    /** Flip the `i`th bit. */
    pub fn flip_i(&mut self, i: usize) {
        self.f[i >> 7] ^= 1 << (i & 127);
    }

    /** Get the `i`th bit. */
    pub fn get_i(&self, i: usize) -> bool {
        (self.f[i >> 7] >> (i & 127)) & 1 == 1
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
        for i in 0..self.f.len() {
            self.f[i] ^= other.f[i];
        }
    }
}


/** `XorLinearEquationSystem` is a system of linear equations, ($\mathbf{Z}^{n}_2$, `Xor`). */
#[derive(Clone, Debug)]
pub struct XorLinearEquationSystem {
    equations: Vec<ZeroOneLinearEquation>,
    equation_size: usize,
    id_bit: Vec<usize>,
}

impl XorLinearEquationSystem {

    /** New a `XorLinearEquationSystem` with `equation_num` `ZeroOneLinearEquation`s and `equation_size` of dimensions. */
    pub fn new(equation_num: usize, equation_size: usize) -> Self {
        XorLinearEquationSystem {
            equations: vec![ZeroOneLinearEquation::new(equation_size); equation_num],
            equation_size: equation_size,
            id_bit: vec![],
        }
    }

    /** New a `ZeroOneLinearEquation` with `equation_size` of dimensions. */
    pub fn new_equation(&mut self) -> ZeroOneLinearEquation {
        ZeroOneLinearEquation::new(self.equation_size)
    }

    /** Set the `j`th bit of the `i`th equation to `1`. */
    pub fn set_i_j(&mut self, i: usize, j: usize) {
        self.equations[i].set_i(j);
    }

    /** Set the `j`th bit of the `i`th equation to `0`. */
    pub fn unset_i_j(&mut self, i: usize, j: usize) {
        self.equations[i].unset_i(j);
    }

    /** Flip the `j`th bit of the `i`th equation. */
    pub fn flip_i_j(&mut self, i: usize, j: usize) {
        self.equations[i].flip_i(j);
    }

    /** Count the number of 1 bits in `i`th equation. */
    pub fn count_ones_i(&self, i: usize) -> usize {
        self.equations[i].f.iter().map(|x| x.count_ones() as usize).sum()
    }

    /** Count the number of 0 bits in `i`th equation. */
    pub fn count_zeros_i(&self, i: usize) -> usize {
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
        self.equations[i].get_i(j)
    }

    /** Check if the basis is full. */
    pub fn is_full_basis(&mut self) -> bool {
        self.equations.len() == self.equation_size
    }

    /** Compute the basis. */
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
            self.id_bit.push(i);

            self.equations.swap(l, j);

            let tmp = self.equations[l].clone();
            
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
    pub fn can_represent(&mut self, mut equation: ZeroOneLinearEquation) -> bool {
        for i in 0..self.equations.len() {
            if equation.get_i(self.id_bit[i]) {
                equation.xor(&self.equations[i]);
            }
        }
        equation.highest_one_bit().is_none()
    }

    /** Insert a new equation into the basis. If the basis is extended, return true, otherwise false. */
    pub fn insert_basis(&mut self, mut equation: ZeroOneLinearEquation) -> bool {
        for i in 0..self.equations.len() {
            if equation.get_i(self.id_bit[i]) {
                equation.xor(&self.equations[i]);
            }
            let highest_one_bit = equation.highest_one_bit();
            if highest_one_bit.is_none() {
                return false;   
            } 
            let highest_one_bit = highest_one_bit.unwrap();
            if highest_one_bit > self.id_bit[i] {
                self.equations.insert(i, equation);
                self.id_bit.insert(i, highest_one_bit);
                return true;
            }
        }
        let highest_one_bit = equation.highest_one_bit();
        if highest_one_bit.is_none() {
            return false;   
        } 
        let highest_one_bit = highest_one_bit.unwrap();
        self.equations.push(equation);
        self.id_bit.push(highest_one_bit);
        true
    }

    /** Get the maximum equation that can be represented by the system. */
    pub fn get_max(&mut self) -> ZeroOneLinearEquation{
        let mut ans = self.equations[0].clone();
        for i in 1..self.equations.len() {
            if !ans.get_i(self.id_bit[i]) {
                ans.xor(&self.equations[i]);
            }
        }
        return ans;
    }

    /** Return the size of the basis. */
    pub fn count_basis(&mut self) -> usize {
        self.equations.len()
    }

}