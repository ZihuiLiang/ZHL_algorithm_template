/** Problem link: https://www.luogu.com.cn/problem/P4718 */
use std::io::stdin;
use zhl_algorithm_template_rs::{math::prime::{PollardRho, MillerRabin, PrimalityTest, ExtractPrimeFactors}, random::generator::{IntGenerator, Pseudorandom64, MT19937_64}};
fn main() {
    let mut rng = MT19937_64::new(None);
    let mut MillerRabin = MillerRabin::new(&mut rng, None);
    let mut rng = MT19937_64::new(None);
    let mut pollard_rho = PollardRho::new(&mut rng, &MillerRabin);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let n: u64 = input.trim().parse().unwrap();
        let primes = pollard_rho.extract_prime_factors_u64(&n);
        if primes.len() == 1 {
            println!("Prime");
        } else {
            println!("{}", primes.iter().max().unwrap());
        }
    }
}