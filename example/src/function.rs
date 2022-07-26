// 函数

fn main() {

    another_function(1, 2);


    let x = 5;

    // 可以用大括号表示一些复杂的表达式
    let y = {
        let x = 3;
        x + 1 // 这里不能加分号，会判定为语句，而不是表达式的值
        // 和内嵌函数不一样，不可以用return
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);


    // 函数可以嵌套
    // 嵌套函数的返回值
    fn five() -> i32  {
        5
        // return 5;
        // 和表达式区别，可以使用return
    };
    println!("five() 的值为: {}", five());

    // 函数的返回值
    println!("add(1, 2) 的值为: {}", add(1, 2));
}

fn another_function(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
    // 不支持自动返回值类型判断
}