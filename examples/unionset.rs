/** Problem link: https://www.luogu.com.cn/problem/P3367 */
use std::io::stdin;
use zhl_algorithm_template_rs::datastructure::unionset::UnionSet;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut split = s.trim().split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let m: usize = split.next().unwrap().parse().unwrap();
    let d: Vec<usize> = (1..=n).map(|x| x.clone()).collect();
    let mut us = UnionSet::new_from_vec(&d);
    for _ in 0..m {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let mut split = s.trim().split_whitespace();
        let op: usize = split.next().unwrap().parse().unwrap();
        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();
        if op == 2 {
            if us.is_union(&x, &y).unwrap() {
                println!("Y");
            } else {
                println!("N");
            }
        } else {
            us.unite(&x, &y).unwrap();
        }
    }
}