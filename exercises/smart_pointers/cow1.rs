// cow1.rs
//
//本练习探讨 Cow 或 Clone-On-Write 类型。 Cow 是一个写时克隆智能指针。
//它可以封装并提供对借用数据的不可变访问，
//当需要突变或所有权时，延迟克隆数据。
//该类型旨在通过 Borrow 特征处理一般借用数据。
//
//
//这个练习的目的是向您展示将数据传递给 Cow 时会发生什么。
//通过检查 TODO 标记处的 Cow::Owned(_) 和 Cow::Borrowed(_) 来修复单元测试。
// 
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        //克隆发生是因为 `input` 需要改变。
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
      //不会发生克隆，因为 `input` 不需要改变.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        //我们也可以传递不带“&”的“slice”，因此 Cow 直接拥有它。
        //在这种情况下，没有发生突变，因此也没有克隆，
        //但结果仍然被拥有，因为它从未被借用或改变。
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        //当然，如果确实发生突变也是如此。
        //在这种情况下，对 `to_mut()` 的调用返回对与之前相同的数据的引用。
        // 
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
