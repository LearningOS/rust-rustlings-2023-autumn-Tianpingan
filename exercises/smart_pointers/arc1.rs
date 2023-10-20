// arc1.rs
//
//在本练习中，我们给出一个 u32 的 Vec，称为“数字”，其值范围为 0 到 99
//--[ 0, 1, 2, ..., 98, 99 ] 我们希望在 8 个不同的线程中同时使用这组数字。
//每个线程将获得每八个值的总和，并带有偏移量。
//
//
//第一个线程（偏移量 0）将对 0, 8, 16, ... 求和
//第二个线程（偏移量 1）将对 1, 9, 17, ... 求和
//第三个线程（偏移量 2）将对 2, 10, 18, ... 求和
//...
//第八个线程（偏移量 7）将对 7, 15, 23, ... 求和
//
//因为我们使用线程，所以我们的值需要是线程安全的。因此，我们使用 Arc。
//我们需要对两个 TODO 中的每一个进行更改。
//
//通过在第一个 TODO 注释所在的位置填写 `shared_numbers` 的值来编译此代码，
//并为第二个 TODO 注释所在的 `child_numbers` 创建一个初始绑定。
//尽量不要创建 `numbers` Vec 的任何副本！
// 
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.


#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
