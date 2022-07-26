// 条件语句

fn main() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }

    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b is {}", b);


    // 类似三目运算符，大括号中是表达式
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}