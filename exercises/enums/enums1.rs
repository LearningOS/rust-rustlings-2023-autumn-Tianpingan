// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String)
}

fn main() {
    println!("{:?}", Message::Quit("asdasd".to_string()));
    println!("{:?}", Message::Echo("asdasd".to_string()));
    println!("{:?}", Message::Move("asdasd".to_string()));
    println!("{:?}", Message::ChangeColor("asdasd".to_string()));
}
