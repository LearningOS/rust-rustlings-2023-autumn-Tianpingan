// tests9.rs
//
//Rust 非常有能力与 C/C++ 和其他静态编译语言共享 FFI 接口，甚至可以在代码本身内链接！
//它通过 extern 块，就像下面的代码一样。
//
//
//`extern` 关键字后面的短字符串指示外部导入函数将遵循哪个 ABI。
//在本练习中，使用“Rust”，同时存在其他变体，例如标准 C ABI 的“C”、Windows ABI 的“stdcall”。
//
//
//外部导入的函数在 extern 块中声明，用分号代替大括号来标记签名结束。
//一些属性可以应用于这些函数声明来修改链接行为，
//如#[link_name = ".."] 修改实际的符号名称。
//
//
//如果要将符号导出到链接环境，也可以使用相同的 ABI 字符串注释在函数定义之前标记 `extern` 关键字。
//Rust 函数的默认 ABI 字面意思是“Rust”，
//因此，如果您想链接纯 Rust 函数，则可以省略整个 extern 术语。
//
//
//Rust 默认情况下会损坏符号，就像 C++ 一样。为了抑制这种行为并使这些函数可以通过名称寻址，
//可以应用属性#[no_mangle]。
//
//在本练习中，您的任务是使测试用例能够调用模块 Foo 中的 `my_demo_function`。
//`my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用相同的函数。
//
//
//除了添加两行属性之外，您不应修改任何现有代码。

#[no_mangle]
extern "C" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    //没有 `extern` 等于 `extern "Rust"`。
    #[no_mangle]
    pub extern "C" fn my_demo_function(a: u32) -> u32 {
        a
    }
    #[no_mangle]
    pub extern "C" fn my_demo_function_alias(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
     //由于其他语言的来源不受信任，外部导入的函数默认是不安全的。
        //您可以将它们包装在安全的 Rust API 中以减轻调用者的负担。
        //
        //
        //安全：我们知道这些函数是安全 Rust 函数的别名。
        // 
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
