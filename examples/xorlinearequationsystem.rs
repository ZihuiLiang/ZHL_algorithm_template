/** Problem link: https://www.luogu.com.cn/problem/P3857 */
use std::io::stdin;
use zhl_algorithm_template_rs::math::linearequation::XorLinearEquationSystem;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut split = s.trim().split_whitespace();
    let mut n = split.next().unwrap().parse::<usize>().unwrap();
    let mut m = split.next().unwrap().parse::<usize>().unwrap();
   
    let mut system = XorLinearEquationSystem::new(m, n);

    for i in 0..m {
        s.clear();
        stdin().read_line(&mut s).unwrap();    
        for (j, a) in s.trim().chars().enumerate() {
            if a == "O".chars().next().unwrap() {
                system.set_i_j(i, j);
            }
        }
    }

    system.compute_basis();
    
    let cnt = system.count_basis();
    let mut ans = 1;
    for _ in 0..cnt {
        ans = (ans << 1) % 2008;
    }

    println!("{}", ans);
}   
