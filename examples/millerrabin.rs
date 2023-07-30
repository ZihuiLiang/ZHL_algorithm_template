/** Problem link: https://www.luogu.com.cn/problem/U82118 */
use std::io::stdin;
use zhl_algorithm_template_rs::{math::prime::{MillerRabin, PrimalityTest, ExtractPrimeFactors}, random::generator::{IntGenerator, Pseudorandom64, MT19937_64}};
fn main() {
    let mut rng = MT19937_64::new(None);
    let mut miller_rabin = MillerRabin::new(&mut rng, None);
    let mut input = String::new();
    while let Ok(x) = stdin().read_line(&mut input) {
        if x == 0 {
            break;
        }
        let n: u128 = input.trim().parse().unwrap();
        if miller_rabin.is_prime(&n) {
            println!("Y");
        } else {
            println!("N");
        }
        input.clear();
    }
}