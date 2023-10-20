//！这是tests7 和tests8 的构建脚本。
//！
//！您应该修改此文件以使两个练习都通过。

fn main() {
    //在tests7中，我们应该设置一个名为`TEST_FOO`的环境变量。
    //在标准输出中打印以让 Cargo 执行此操作。
    //
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); //这里这个时间戳有什么用呢？
    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:{}", your_command);


    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
