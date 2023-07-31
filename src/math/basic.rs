
/** Return (`a` + `b`) mod `c`*/
#[inline(always)]
pub fn add_mod_u32(a: &u32, b: &u32, c: &u32) -> u32 {
    if *a >= *c - *b {
        *a - (*c - *b)
    } else {
        *a + *b
    }
}

/** Return (`a` + `b`) mod `c`*/
#[inline(always)]
pub fn add_mod_u64(a: &u64, b: &u64, c: &u64) -> u64 {
    if *a >= *c - *b {
        *a - (*c - *b)
    } else {
        *a + *b
    }
}
/** Return (`a` - `b`) mod `c`*/
#[inline(always)]
pub fn sub_mod_u32(a: &u32, b: &u32, c: &u32) -> u32 {
    if *a >= *b {
        *a - *b
    } else {
        *a + (*c - *b)
    }
}

/** Return (`a` - `b`) mod `c`*/
#[inline(always)]
pub fn sub_mod_u64(a: &u64, b: &u64, c: &u64) -> u64 {
    if *a >= *b {
        *a - *b
    } else {
        *a + (*c - *b)
    }
}

/** Return (`a` * `b`) mod `c`*/
#[inline(always)]
pub fn mul_mod_u32(a: &u32, b: &u32, c: &u32) -> u32 {
    ((*a as u64) * (*b as u64) % (*c as u64)) as u32
}

/** Return (`a` * `b`) mod `c`*/
#[inline(always)]
pub fn mul_mod_u64(a: &u64, b: &u64, c: &u64) -> u64 {
    ((*a as u128) * (*b as u128) % (*c as u128)) as u64
}

/** `pow_mod_u32` is an algorithm to compute $a^b \mod c$ in $O(\log(b))$ time*/
pub fn pow_mod_u32(a: &u32, b: &u32, c: &u32) -> u32 {
    let mut ans = 1u64;
    let mut a = *a as u64;
    let mut b = *b as u64;
    let c = *c as u64;
    while b > 0 {
        if b & 1 == 1 {
            ans = (ans * a) % c;
        }
        a = (a * a) % c;
        b >>= 1;
    }
    ans as u32
}

/** `pow_mod_u64` is an algorithm to compute $a^b \mod c$ in $O(\log(b))$ time*/
pub fn pow_mod_u64(a: &u64, b: &u64, c: &u64) -> u64 {
    let mut ans = 1u128;
    let mut a = *a as u128;
    let mut b = *b as u128;
    let c = *c as u128;
    while b > 0 {
        if b & 1 == 1 {
            ans = (ans * a) % c;
        }
        a = (a * a) % c;
        b >>= 1;
    }
    ans as u64
}

/** `gcd_u32` is an algorithm to compute the greatest common divisor of two integers $a$ and $b$ in $O(\log (a+b))$ */
pub fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/** `gcd_u64` is an algorithm to compute the greatest common divisor of two integers $a$ and $b$ in $O(\log (a+b))$ */
pub fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/** `gcd_u128` is an algorithm to compute the greatest common divisor of two integers $a$ and $b$ in $O(\log (a+b))$ */
pub fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub mod from_to {
/*! This crate demonstrates the algorithms for transforming data types:
 * 1. `uxix` is an algorithm to transform `ux` to `ix` by decreasing $2^{x-1}$.
 * 2. `ixux` is an algorithm to transform `ix` to `ux` by increasing $2^{x-1}$.
 * Note that by applying `uxix` and `ixux`, the partial order is preserved.
 * 3. `default_uxix` is an algorithm to transform `ux` to `ix` by keeping the same bits.
 * 4. `default_ixux` is an algorithm to transform `ix` to `ux` by keeping the same bits.
*/

    #[inline(always)]
    pub fn u8i8(x: u8) -> i8 {
        if x >= 128 {
            (x - 128) as i8
        } else {
            (x as i8) - 127 - 1
        }
    }

    #[inline(always)]
    pub fn i8u8(x: i8) -> u8 {
        if x < 0 {
            (x + 127 + 1) as u8
        } else {
            (x as u8) + 128
        }
    }

    #[inline(always)]
    pub fn u16i16(x: u16) -> i16 {
        if x >= 32768 {
            (x - 32768) as i16
        } else {
            (x as i16) - 32767 - 1
        }
    }

    #[inline(always)]
    pub fn i16u16(x: i16) -> u16 {
        if x < 0 {
            (x + 32767 + 1) as u16
        } else {
            (x as u16) + 32768
        }
    }
    
    #[inline(always)]
    pub fn u32i32(x: u32) -> i32 {
        if x >= 2147483648 {
            (x - 2147483648) as i32
        } else {
            (x as i32) - 2147483647 - 1
        }
    }

    #[inline(always)]
    pub fn i32u32(x: i32) -> u32 {
        if x < 0 {
            (x + 2147483647 + 1) as u32
        } else {
            (x as u32) + 2147483648
        }
    }

    #[inline(always)]
    pub fn u64i64(x: u64) -> i64 {
        if x >= 9223372036854775808 {
            (x - 9223372036854775808) as i64
        } else {
            (x as i64) - 9223372036854775807 - 1
        }
    }

    #[inline(always)]
    pub fn i64u64(x: i64) -> u64 {
        if x < 0 {
            (x + 9223372036854775807 + 1) as u64
        } else {
            (x as u64) + 9223372036854775808
        }
    }

    #[inline(always)]
    pub fn u128i128(x: u128) -> i128 {
        if x >= 170141183460469231731687303715884105728 {
            (x - 170141183460469231731687303715884105728) as i128
        } else {
            (x as i128) - 170141183460469231731687303715884105727 - 1
        }
    }

    #[inline(always)]
    pub fn i128u128(x: i128) -> u128 {
        if x < 0 {
            (x + 170141183460469231731687303715884105727 + 1) as u128
        } else {
            (x as u128) + 170141183460469231731687303715884105728
        }
    }
    
    #[inline(always)]
    pub fn default_u8i8(x: u8) -> i8 {
        if x <= 127 {
            x as i8
        } else {
            (x - 128) as i8 - 127 - 1
        }
    }

    #[inline(always)]
    pub fn default_i8u8(x: i8) -> u8 {
        if x >= 0 {
            x as u8
        } else {
            (x + 127 + 1) as u8 + 128
        }
    }

    #[inline(always)]
    pub fn default_u16i16(x: u16) -> i16 {
        if x <= 32767 {
            x as i16
        } else {
            (x - 32768) as i16 - 32767 - 1
        }
    }

    #[inline(always)]
    pub fn default_i16u16(x: i16) -> u16 {
        if x >= 0 {
            x as u16
        } else {
            (x + 32767 + 1) as u16 + 32768
        }
    }

    #[inline(always)]
    pub fn default_u32i32(x: u32) -> i32 {
        if x <= 2147483647 {
            x as i32
        } else {
            (x - 2147483648) as i32 - 2147483647 - 1
        }
    }

    #[inline(always)]
    pub fn default_i32u32(x: i32) -> u32 {
        if x >= 0 {
            x as u32
        } else {
            (x + 2147483647 + 1) as u32 + 2147483648
        }
    }

    #[inline(always)]
    pub fn default_u64i64(x: u64) -> i64 {
        if x <= 9223372036854775807 {
            x as i64
        } else {
            (x - 9223372036854775808) as i64 - 9223372036854775807 - 1
        }
    }

    #[inline(always)]
    pub fn default_i64u64(x: i64) -> u64 {
        if x >= 0 {
            x as u64
        } else {
            (x + 9223372036854775807 + 1) as u64 + 9223372036854775808
        }
    }

    #[inline(always)]
    pub fn default_u128i128(x: u128) -> i128 {
        if x <= 170141183460469231731687303715884105727 {
            x as i128
        } else {
            (x - 170141183460469231731687303715884105728) as i128 - 170141183460469231731687303715884105727 - 1
        }
    }

    #[inline(always)]
    pub fn default_i128u128(x: i128) -> u128 {
        if x >= 0 {
            x as u128
        } else {
            (x + 170141183460469231731687303715884105727 + 1) as u128 + 170141183460469231731687303715884105728
        }
    }
}