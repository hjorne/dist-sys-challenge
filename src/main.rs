use std::io;

use fly_challenge::handler::Handler;
use fly_challenge::messages::message::Message;

fn main() {
    let mut handler = Handler::default();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let message = serde_json::from_str::<Message>(&dbg!(input)).unwrap();
        if let Some(response) = handler.handle(dbg!(message)) {
            let json = serde_json::to_string(&dbg!(response)).unwrap();
            println!("{}", dbg!(&json));
        }
    }
}
