/*! This crate demonstrates the algorithms for solving string problems.
*/


/** `KMP` is an algorithm to compute the prefix-function for a model string `s` in $O(|s|)$ time. `pref_func[i]` is the length of the longest border (the border should be shorter than $s[0..=i]$) of $s[0..]$ and $s[..=i]$. Also, given another string `t`, for $i\in [0,|t|)$ `KMP` can be applied to compute the longest border (the border can be $s[0..=i]$ and $t[0..=i]$) of $s[0..]$ and $t[..=i]$ in $O(|t|)$ time (see `extend` function). */
#[derive(Clone, Debug)]
pub struct KMP<T: Clone + std::fmt::Debug + std::cmp::PartialEq> {
    pref_func: Vec<usize>,
    s: Vec<T>,
}

impl<T: Clone + std::fmt::Debug + std::cmp::PartialEq> KMP<T> {

    /** Return `KMP` with computed prefix-function in $O(|s|)$ time. */
    pub fn new(s: &Vec<T>) -> Self {
        if s.len() == 0 {
            return KMP { pref_func: vec![], s: vec![] };
        }
        let mut pref_func = vec![0; s.len()];
        for i in 1..s.len() {
            let mut j = pref_func[i - 1];
            while j > 0 && s[j] != s[i] {
                j = pref_func[j - 1];
            }
            if s[j] == s[i] {
                j += 1;
            }
            pref_func[i] = j;
        }
        KMP { pref_func, s: s.clone() }
    }

    /** Return prefix-function. */
    pub fn get_pref_func(&self) -> &Vec<usize> {
        &self.pref_func
    }

    /** Return the model string. */
    pub fn get_s(&self) -> &Vec<T> {
        &self.s
    }

    /** Compute the prefix-function of `t` in `s`. */
    pub fn extend(&self, t: &Vec<T>) -> Vec<usize> {
        if t.len() == 0 {
            return vec![];
        }
        let mut ans = vec![0;t.len()];
        let mut j = 0;
        for i in 0..t.len() {
            if j == self.s.len() {
                j = self.pref_func[j - 1];
            }
            while j > 0 && self.s[j] != t[i] {
                j = self.pref_func[j - 1];
            }
            if self.s[j] == t[i] {
                j += 1;
            }
            ans[i] = j;
        }
        ans
    }

    /** Return the all occurrences of `s` in `t` (Return the `i`s with `t_pref_func[i+s.len()-1]=s.len()` where `t_pref_func` is computed by calling `extend(t)`. ). */
    pub fn find_occurences(&self, t: &Vec<T>) -> Vec<usize> {
        if t.len() < self.s.len() || t.len() == 0 {
            return vec![];
        }
        self.extend(t).iter().enumerate().filter(|(_, &x)| x == self.s.len()).map(|(i, _)| i - self.s.len() + 1).collect()
    }
}


/** `EXKMP` is an algorithm to compute the z-function for a model string `s` in $O(|s|)$ time. $z[i]$ is the length of the longest common prefix of $s[0..]$ and $s[i..]$. Also, given another string `t`, for $i\in [0,|t|)$ `EXKMP` can be applied to compute the longest common prefix of $s[0..]$ and $t[i..]$ in $O(|t|)$ time (see `extend` function). */
#[derive(Clone, Debug)]
pub struct EXKMP<T: Clone + std::fmt::Debug + std::cmp::PartialEq> {
    z: Vec<usize>,
    s: Vec<T>,
}

impl<T: Clone + std::fmt::Debug + std::cmp::PartialEq> EXKMP<T> {

    /** Return `EXKMP` with computed z-function in $O(|s|)$ time. */
    pub fn new(s: &Vec<T>) -> Self {
        if s.len() == 0 {
            return EXKMP { z: vec![], s: vec![] };
        }
        let mut z = vec![0; s.len()];
        z[0] = s.len();
        let mut l = 0;
        let mut r = 0;
        for i in 1..s.len() {
            if i <= r && z[i - l] < r - i + 1 {
                z[i] = z[i - l];
            } else {
                let mut j = if i > r {0} else {r - i + 1};
                while i + j < s.len() && s[j] == s[i + j] {
                    j += 1;
                }
                z[i] = j;
                l = i;
                r = i + z[i] - 1;
            }
        }      
        EXKMP { z, s: s.clone() }
    }

    /** Return z-function. */
    pub fn get_z(&self) -> &Vec<usize> {
        &self.z
    }

    /** Return the model string. */
    pub fn get_s(&self) -> &Vec<T> {
        &self.s
    }

    /** Compute the longest common prefix of model string and each substring of `t` and return the vector of the lengths in $O(|t|)$. */
    pub fn extend(&self, t: &Vec<T>) -> Vec<usize> {
        let mut ans = vec![0;t.len()];
        if t.len() == 0 {
            return ans;
        }
        let mut l = 0;
        let mut r = 0;
        while r < self.s.len() && r < t.len() && self.s[r] == t[r] {
            r += 1;
        }
        ans[0] = r;
        r = if r > 0 {r - 1} else {0};
        for i in 1..t.len() {
            if i <= r && self.z[i - l] < r - i + 1 {
                ans[i] = self.z[i - l];
            } else {
                let mut j = if i > r {0} else {r - i + 1};
                while i + j < t.len() && j < self.s.len() && t[i + j] == self.s[j] {
                    j += 1;
                }
                ans[i] = j;
                l = i;
                r = i + ans[i] - 1;
            }
        }
        ans
    }
}