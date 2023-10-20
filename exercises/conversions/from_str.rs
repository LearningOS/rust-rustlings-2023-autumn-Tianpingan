// from_str.rs
//
//这与 from_into.rs 类似，但这次我们将实现 `FromStr` 和
//返回错误而不是回退到默认值。此外，
//实现 FromStr 后，您可以对字符串使用“parse”方法来生成实现者类型的对象。
//您可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html 阅读更多相关信息
//
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

//我们将在 `FromStr` 实现中使用此错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    //空输入字符串
    Empty,
    //字段数量不正确
    BadLen,
    //名称字段为空
    NoName,
   //来自 parse::<usize>() 的包装错误
    ParseInt(ParseIntError),
}
//脚步：
//1. 如果提供的字符串长度为0，则返回错误
//2. 用逗号分割给定的字符串
//3. split只能返回2个元素，否则返回错误
//
//4. 从 split 操作中提取第一个元素并将其用作名称
//5. 从 split 操作中提取另一个元素，并将其解析为 `usize` 作为年龄，类似于 `"4".parse::<usize>()`
//
//6. 如果在提取姓名和年龄时出现问题，则应返回错误
//
//如果一切顺利，则返回 Person 对象的 Result
//
//顺便说一句： `Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着如果你想返回一个字符串错误消息，
//你可以通过使用 return `Err("my error message".into())` 来做到这一点。
// 

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s=="" {
            Err(ParsePersonError::Empty)
        } else {
            let parts: Vec<&str> = s.split(",").collect();
            let name = parts[0].trim();
            if parts.len()!=2 {
                Err(ParsePersonError::BadLen)
            } else if name=="" {
                Err(ParsePersonError::NoName)
            } else {
                let age = parts[1].trim();
                let age = age.parse::<usize>().map_err(ParsePersonError::ParseInt)?;
                Ok(Person{name: parts[0].to_string(), age: parts[1].parse::<usize>().unwrap()})
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
