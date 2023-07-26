/** Problem link: https://www.luogu.com.cn/problem/P4570 */
use std::io::stdin;
use zhl_algorithm_template_rs::math::XorLinearEquationSystem;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut n = s.trim().parse::<usize>().unwrap();

    let mut system = XorLinearEquationSystem::new(0, 64);
    let mut d = (0..n).map(|_| {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let mut split = s.trim().split_whitespace();
        let mut a = split.next().unwrap().parse::<i64>().unwrap();
        let mut b = split.next().unwrap().parse::<i32>().unwrap();
        let mut equation = system.new_equation();
        for i in 0..64 {
            if a & 1 == 1 {
                equation.set_i(i);
            }
            a >>= 1;
        }
        (equation, b)
    }).collect::<Vec<_>>();

    d.sort_by(|a, b| b.1.cmp(&a.1));

    let mut ans = 0;

    for i in 0..n {
        if !system.can_represent(d[i].0.clone()) {
            ans += d[i].1;
            system.insert_basis(d[i].0.clone());
        }
    }


    println!("{}", ans);
}   
