// clippy1.rs
//
//Clippy 工具是用于分析代码的 lint 集合，以便您可以捕获常见错误并改进 Rust 代码。
//
//
//对于这些练习，当出现 Clippy 警告时，代码将无法编译，请检查输出中的 Clippy 建议以解决练习。
// 
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


use std::f32::consts::PI;

fn main() {
    let pi = PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
