// main.rs

// 声明a模块，a就是a.rs的文件名称
mod a;

// 使用use指定使用a模块中的哪些内容
// 注意：只能使用a模块中使用pub关键字导出的
// use a::Person;
// 也可以写完整的导入路径
// use crate::a::foo;
// 导入多个
use a::{Person, foo};
// 或者*匹配所有导出的成员
// use a::*;

fn main() {
    let p = Person;
    println!("{}", foo());
}