// tests5.rs
//
//Rust 中的“不安全”充当合约。
//
//当在项声明上标记“不安全”时，例如函数，
//一个特质等等，它旁边声明了一个契约。然而，
//合约的内容不能仅用单个关键字来表达。
//因此，您有责任在`# Safety`中手动声明它
//文档中对该项目的评论部分。
//
//当大括号括起来的代码块上标记了“不安全”时，
//它声明遵守某些契约，例如某些指针参数的有效性，
//某些内存地址的所有权。然而，就像上面的文本一样，您仍然需要在代码块的注释中说明如何遵守契约。
//
//
//
//注意：所有注释都是为了代码的可读性和可维护性，
//而 Rust 编译器将其对代码健全性的信任交给了你自己！
//如果无法证明我们自己代码的内存安全性和健全性，
//退后一步，使用安全代码！
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// “address”必须包含对有效“u32”值的可变引用。
unsafe fn modify_by_address(address: usize) {
//TODO：填写下面代码块的安全注意事项，以匹配您的代码行为和该函数的约定。
    //您可以使用下面测试的注释作为格式参考。
    //
    let ptr = address as *mut u32;
    unsafe {
        *ptr = 0xAABBCCDD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        //安全：地址保证有效，并且包含对“u32”局部变量的唯一引用。
        // 
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
