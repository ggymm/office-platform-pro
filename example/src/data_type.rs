// 数据类型

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x is {}", x);
    println!("y is {}", y);

    // 元组
    // 可以包含不同数据类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    println!("0 is {} 1 is {} 2 is {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    // y 等于 6.4
    println!("x is {} y is {} z is {}", x, y, z);


    // 数组
    // 同类型数据
    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组
    println!("a is {:?}", a);

    let b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组
    println!("a is {:?}", b)
}