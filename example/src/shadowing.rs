// 重影

fn main() {

    // 重影就是指变量的名称可以被重新使用的机制
    // 相当于，用了同一个名字代表另一个实体
    // 类型，可变属性，值都可以被改变
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 与可变变量不同，可变变量只能修改值
}