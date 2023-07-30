use zhl_algorithm_template_rs::random::generator::{MT19937_64, Pseudorandom64};
fn main() {
    println!("Generate 10 random 64-bit unsigned integers with system seed:");
    let mut gen = MT19937_64::new(None);
    for _ in 0..10 {
        println!("{}", gen.gen());
    }
    
    println!("Generate 10 random 64-bit unsigned integers with fixed seed=0:");
    let mut gen = MT19937_64::new(Some(0));
    for _ in 0..10 {
        println!("{}", gen.gen());
    }
}