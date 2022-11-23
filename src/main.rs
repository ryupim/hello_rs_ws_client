use tungstenite::{connect, Message};
use url::Url;
use serde_json;

fn main() {
    let (mut socket, _response) = connect(Url::parse("ws://localhost:8765").unwrap()).expect("Cant connect");

    loop {
        println!("Please input some strings");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).ok();
        let answer = word.trim().to_string();
        socket.write_message(Message::Text(answer)).unwrap();

        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Cant parse to JSON");
        println!("{:?}", parsed["result"]);

    }
}
