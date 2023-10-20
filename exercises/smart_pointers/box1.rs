// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. 
// This becomes problematic for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap, 
// which also allows us to wrap a recursive type. 
// 
//
//我们在本练习中实现的递归类型是 `cons list`
//-函数式编程语言中常见的数据结构。
//cons 列表中的每个项目都包含两个元素：当前项目的值和下一个项目的值。
//最后一项是一个名为“Nil”的值。
//
//第 1 步：在枚举定义中使用 `Box` 来使代码编译
//步骤 2：通过替换 `todo!()` 创建空和非空 cons 列表
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.


#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(3, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
