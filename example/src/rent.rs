// 租借
// 其实就是引用，指针
// 租借不能获取值的所有权

fn main() {
    let s1 = String::from("hello");
    let mut s2 = &s1;

    // let s3 = s1;
    // println!("{}", s2)
    // 不正确，因为s2的所有权被s3获取了，所以s2无效

    let s3 = s1;
    s2 = &s3;
    // 如果要用s2，需要从s3重新租借


    let s4 = String::from("run");
    let s5 = &s4;
    println!("{}", s5);
    // s5.push_str("oob"); // 错误，禁止修改租借的值
    // println!("{}", s5);

    let mut s6 = String::from("run");
    // s6 是可变的
    let s7 = &mut s6;
    // s7 是可变的引用
    s7.push_str(" oob");
    println!("{}", s7);
    // 必须原值和引用都是可变的，才可以修改
}