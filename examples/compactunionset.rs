/** Problem link: https://www.luogu.com.cn/problem/P3367 */
use std::io::stdin;
use zhl_algorithm_template_rs::datastructure::CompactUnionSet;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut split = s.trim().split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let m: usize = split.next().unwrap().parse().unwrap();
    let mut us = CompactUnionSet::new(n);
    for _ in 0..m {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let mut split = s.trim().split_whitespace();
        let op: usize = split.next().unwrap().parse().unwrap();
        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();
        if op == 2 {
            if us.is_union(x - 1, y - 1).unwrap() {
                println!("Y");
            } else {
                println!("N");
            }
        } else {
            us.unite(x - 1, y - 1).unwrap();
        }
    }
}