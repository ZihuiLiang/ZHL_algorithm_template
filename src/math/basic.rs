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