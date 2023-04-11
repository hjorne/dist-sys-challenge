use std::sync::mpsc;
use std::time::Duration;
use std::{io, thread};

use fly_challenge::handler::Handler;
use fly_challenge::messages::message::Message;
use fly_challenge::syncer::Syncer;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handler = Handler::new(tx);
    let mut syncer = Syncer::new(rx);
    thread::spawn(move || loop {
        syncer.run();
        thread::sleep(Duration::from_millis(250));
    });
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
