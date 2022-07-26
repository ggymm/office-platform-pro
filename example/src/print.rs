// 打印语句

fn main() {
    let a = 1;

    // 有感叹号的是宏规则，不是函数
    println!("{{}}");
    println!("a is {}", a);
    println!("a is {0}, a again is {0}", a);
}