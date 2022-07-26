// 所有权

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1); // 错误！s1 已经失效

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}