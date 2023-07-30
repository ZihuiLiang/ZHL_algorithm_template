/** Problem link: https://www.luogu.com.cn/problem/P3375 */
use std::io::stdin;
use zhl_algorithm_template_rs::string::kmp::KMP;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let a = s.trim().chars().collect::<Vec<_>>();
    s.clear();
    stdin().read_line(&mut s).unwrap();
    let b = s.trim().chars().collect::<Vec<_>>();
    let mut kmpb = KMP::new(&b);
    let d = kmpb.find_occurences(&a);
    for i in d {
        println!("{}", i + 1);
    }
    for (i, v) in kmpb.get_pref_func().iter().enumerate() {
        print!("{}", v);
        if i != b.len() - 1 {
            print!(" ");
        }
    }
    println!();
}