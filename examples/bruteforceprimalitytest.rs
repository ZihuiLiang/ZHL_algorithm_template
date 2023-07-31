/** Problem link: https://www.luogu.com.cn/problem/P1075 */
use std::io::stdin;
use zhl_algorithm_template_rs::math::prime::{BruteForcePrimalityTest, ExtractPrimeFactors, PrimalityTest};
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    println!("{}", BruteForcePrimalityTest::new().extract_prime_factors_u64(&n).iter().max().unwrap());
}