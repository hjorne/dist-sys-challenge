use std::io;

use fly_challenge::handler::Handler;
use fly_challenge::messages::message::Message;

fn main() {
    let mut handler = Handler::default();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let message = serde_json::from_str::<Message>(&input).unwrap();
        match message {
            Message::Request(request) => {
                let response = handler.handle_request(request);
                let json = serde_json::to_string(&response).unwrap();
                println!("{}", &json);
            }
            Message::Response(response) => handler.handle_response(response),
        }
    }
}
