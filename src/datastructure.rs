/*! This crate demonstrates the datastructures
*/

use std::collections::HashMap;

/** `UnionSet` is a datastructure for maintaining the set relation between elements. It supports the operations as follows:
    + `contains(x)` returns whether `x` is in the datastructure. ($O(1)$ time per operation)
    + `insert(x)` inserts `x` into the datastructure. ($O(1)$ time per operation)
    + `getfa(x)` returns the father of `x`. Here father means the representative of the set containing `x`. (amortized $O(1)$ time per operation)
    + `is_union(x,y)` returns whether `x` and `y` are in the same set. (amortized $O(1)$ time per operation)
    + `unite(x,y)` unites the set containing `x` and the set containing `y`. (amortized $O(1)$ time per operation)
*/
pub struct UnionSet<T: Clone + std::fmt::Debug + std::hash::Hash + std::cmp::Eq> {
    fa: HashMap<T, T>,  
}

impl<T: Clone + std::fmt::Debug + std::hash::Hash + std::cmp::Eq> UnionSet<T> {

    /** New an empty `UnionSet`*/
    pub fn new()-> Self {
        UnionSet {
            fa: HashMap::new(),
        }
    }

    /** New a `UnionSet` with elements from vector `x` where each element will be initialized as an individual set. */
    pub fn new_from_vec(x: &Vec<T>) -> Self {
        UnionSet {
            fa: x.iter().map(|x| (x.clone(), x.clone())).collect(),
        }
    }

    /** `contains(x)` returns whether `x` is in the datastructure. ($O(1)$ time per operation)*/
    pub fn contains(&self, x: &T) -> bool {
        self.fa.contains_key(x)
    }

    /** `insert(x)` inserts `x` into the datastructure as an individual set. ($O(1)$ time per operation)*/
    pub fn insert(&mut self, x: &T) -> Result<(), ()> {
        if self.fa.contains_key(x) {
            return Err(());
        }
        self.fa.insert(x.clone(), x.clone());
        return Ok(());
    }


    fn _getfa(&mut self, x: &T) -> T {
        let mut y = self.fa[x].clone();
        if y != *x {
            y = self._getfa(&y);
            *self.fa.get_mut(x).unwrap() = y.clone();
        }
        return y;
    }

    /** `getfa(x)` returns the father of `x`. Here father means the representative of the set containing `x`. (amortized $O(1)$ time per operation)*/
    pub fn getfa(&mut self, x: &T) -> Result<T, ()>{
        if !self.fa.contains_key(x) {
            return Err(());
        }
        Ok(self._getfa(x))
    }

    /** `is_union(x,y)` returns whether `x` and `y` are in the same set. (amortized $O(1)$ time per operation)*/
    pub fn is_union(&mut self, x: &T, y: &T) -> Result<bool, ()> {
        if !self.fa.contains_key(x) || !self.fa.contains_key(y) {
            return Err(());
        }
        Ok(self._getfa(x) == self._getfa(y))
    }

    /** `unite(x,y)` unites the set containing `x` and the set containing `y`. (amortized $O(1)$ time per operation)*/
    pub fn unite(&mut self, x: &T, y: &T) -> Result<(),()> {
        if !self.fa.contains_key(x) || !self.fa.contains_key(y) {
            return Err(());
        } 
        let xfa = self._getfa(x);
        let yfa = self._getfa(y);
        *self.fa.get_mut(&xfa).unwrap() = yfa;
        Ok(())
    }
}

/** `CompactUnionSet` is a special `UnionSet` where the elements are sequential as $\\{0,..,n-1\\}$.*/
pub struct CompactUnionSet {
    fa: Vec<usize>,
}

impl CompactUnionSet {
    
    pub fn new(n: usize) -> Self {
        CompactUnionSet {
            fa: (0..n).collect(),
        }
    }

    pub fn enlarge_to(&mut self, n: usize) {
        if n <= self.fa.len() {
            return;
        }
        self.fa.resize(n, 0);
        for i in self.fa.len()..n {
            self.fa[i] = i;
        }
    }

    fn _getfa(&mut self, x: usize) -> usize {
        let mut fa = self.fa[x];
        if fa != x {
            fa = self._getfa(fa);
            self.fa[x] = fa;
        }
        return fa;
    }

    pub fn getfa(&mut self, x: usize) -> Result<usize, ()> {
        if x >= self.fa.len() {
            return Err(());
        }
        Ok(self._getfa(x))
    }


    pub fn is_union(&mut self, x: usize, y: usize) -> Result<bool, ()> {
        if x >= self.fa.len() || y >= self.fa.len() {
            return Err(());
        }
        Ok(self.getfa(x) == self.getfa(y))
    }

    pub fn unite(&mut self, x: usize, y: usize) -> Result<(), ()> {
        if x >= self.fa.len() || y >= self.fa.len() {
            return Err(());
        }
        let xfa = self._getfa(x);
        let yfa = self._getfa(y);
        self.fa[xfa] = yfa;
        Ok(())
    }
}
