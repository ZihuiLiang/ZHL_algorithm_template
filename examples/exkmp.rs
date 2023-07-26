/** Problem link: https://www.luogu.com.cn/problem/P5410 */
use std::io::stdin;
use zhl_algorithm_template_rs::string::EXKMP;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    stdin().read_line(&mut a).unwrap();
    let mut a: Vec<u8> = a.trim().as_bytes().to_vec();
    stdin().read_line(&mut b).unwrap();
    let mut b: Vec<u8> = b.trim().as_bytes().to_vec();
    let exkmp = EXKMP::new(&b);
    let mut ans0 = 0i64;
    let z = exkmp.get_z();
    for (i, j) in z.iter().enumerate() {
        ans0 ^= (i + 1) as i64 * (j + 1) as i64;
    }
    let mut ans1 = 0i64;
    let z = exkmp.extend(&a);
    for (i, j) in z.iter().enumerate() {
        ans1 ^= (i + 1) as i64 * (j + 1) as i64;
    }
    println!("{ans0}\n{ans1}");
}