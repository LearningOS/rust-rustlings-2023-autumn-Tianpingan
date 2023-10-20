// from_into.rs
//
//From 特征用于值到值的转换。如果某个类型正确实现了 From，
//Into 特征应该相反地工作。您可以在 https://doc.rust-lang.org/std/convert/trait.From.html 阅读更多相关信息
//
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

//我们实现了 Default 特征，以便在提供的字符串无法转换为 Person 对象时将其用作后备
// 
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

//您的任务是完成此实现，以便编译 `let p = Person::from("Mark,20")` 行请注意，您需要使用以下命令将年龄组件解析为 `usize`类似于“4”.parse::<usize>()`。
//需要适当处理此结果。
//
//
//
//脚步：
//1. 如果提供的字符串长度为0，则返回默认的Person。
//2. 用逗号分割给定的字符串。
//3. 从 split 操作中提取第一个元素并将其用作名称。
//4. 如果名称为空，则返回默认的Person。
//5. 从 split 操作中提取另一个元素并将其解析为 `usize` 作为年龄。
//
//如果在解析年龄时出现问题，则返回默认的 Person 否则，则返回一个实例化的 Person 对象以及结果
// 



impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len()==2 && parts[0] != "" && parts[1] != "" && parts[0].chars().all(char::is_alphabetic) && parts[1].chars().all(char::is_numeric) {
            Person {name: parts[0].to_string(), age: parts[1].parse::<usize>().unwrap()}
        } else {
            Person::default()
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
