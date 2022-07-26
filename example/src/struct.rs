// 结构体


// 结构体必须拥有成员变量的所有权
// 因为结构体释放时会释放所有成员变量
// 所以字符串应该定义为String而不是str
// 同理，使用结构体成员变量赋值时。也要注意所有权问题，不能直接赋值，而应该用clone()
struct Site {
    domain: String,
    name: String,
    found: u32,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

fn main() {
    // 实例化
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        found: 2013,
    };


    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        found: 0,
    };

    // 元组结构体，定义了类型的元组
}