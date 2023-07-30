/** Problem link: https://www.luogu.com.cn/problem/P3812 */
use std::io::stdin;
use zhl_algorithm_template_rs::math::linearequation::XorLinearEquationSystem;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut n = s.trim().parse::<usize>().unwrap();
   
    let mut system = XorLinearEquationSystem::new(n, 51);

    s.clear();
    stdin().read_line(&mut s).unwrap();    
    for (i, a) in s.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).enumerate() {
        for j in 0..51 {
            if a & (1 << j) != 0 {
                system.set_i_j(i, j);
            }
        }
    }

    system.compute_basis();
    
    let mx = system.get_max();
    let mut ans = 0i64; 
    for i in 0..51 {
        if mx.get_i(i) {
            ans += 1i64 << i;
        }
    }
    println!("{}", ans);
}   
