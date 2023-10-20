// tests7.rs
//
//构建包时，有些依赖项既不能在`Cargo.toml`中导入，也不能直接链接；
//一些预处理因代码生成和设置特定于包的配置而异。
//
//
//Cargo 的目的并不是取代其他构建工具，但它确实通过名为“build.rs”的自定义构建脚本与它们集成。
//该文件通常放置在项目的根目录中，而在本例中与本练习的目录相同。
//
//
//
//它可用于：
//
//-构建捆绑的 C 库。
//-在主机系统上查找 C 库。
//-根据规范生成 Rust 模块。
//-执行 crate 所需的任何特定于平台的配置。
//
//设置配置时，我们可以在构建脚本中使用 `println!` 来告诉 Cargo 遵循一些指令。
//通用格式为：
//
//println!("货物:{}", your_command_in_string);
//
//有关构建脚本的更多信息，请参阅官方 Cargo 书籍
//信息：
//https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
//在本练习中，我们寻找一个环境变量并期望它落在一个范围内。
//您可以查看测试用例以了解详细信息。
//
//你不应该修改这个文件。修改同一目录中的`build.rs`以通过此练习。
//
//
//执行 `rustlingshinttests7` 或使用 `hint`watch 子命令
//暗示。


fn main() {}

#[cfg(test)]
mod tests {
 

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
