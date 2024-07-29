// a.rs
// 使用 pub 导出结构体
pub struct Person;

// 导出函数
pub fn foo() -> String {
    String::from("I am a coder")
}

// 没有使用pub导出的数据不能被外部使用
enum Color {
    Red(u8, u8, u8),
    Green(u8, u8, u8),
    Blue(u8, u8, u8),
}