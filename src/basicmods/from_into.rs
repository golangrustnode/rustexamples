use std::convert::From;

/*
参考文档：
https://doc.rust-lang.org/std/convert/trait.Into.html
https://blog.csdn.net/wsp_1138886114/article/details/109162993
One should avoid implementing Into and implement From instead. Implementing From automatically provides one with an implementation of Into thanks to the blanket implementation in the standard library.
通过实现From，Into会自动实现。
*/

#[derive(Debug)]
pub struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn frommain() {
    let num = Number::from(32);
    println!("My Number num is {:#?}", num);
    let init_0 = 6;
    let num2: Number = init_0.into();
    println!("My Number-num2 is {:#?}", num2);
    let my_str = "hello";
    let my_string = String::from(my_str);
}
