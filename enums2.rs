#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize {width: u8, height: u8},
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },

    ];

    for message in &messages {
        message.call();
    }
}
