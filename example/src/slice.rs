// 切片
// 切片是数据的视图

// x..y 表示 [x, y)
// ..y 等价于 0..y
// x.. 等价于位置 x 到数据结束
// .. 等价于位置 0 到结束

fn main() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    let mut s1 = String::from("test");
    let slice1 = &s1[0..3];
    s1.push_str("test"); // 错误，被引用的字符串切片不能修改
    println!("{}", slice1); // 引用部分必须生效（被使用）


    // str 是 Rust核心语言类型（不可变）
    let s = "hello"; // s 是 &str类型的变量

    // String 是 标准库提供的数据类型，支持更多的功能，包括追加，清空等操作（可变）

    // 总结，如果作为参数，可以用&str
    // 如果要有所有权，就用String
}