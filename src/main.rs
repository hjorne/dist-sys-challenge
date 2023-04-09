use std::io;
use std::io::Write;

use fly_challenge::handler::Handler;
use fly_challenge::message::Request;

fn main() {
    let mut handler = Handler::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let request = serde_json::from_str::<Request>(&dbg!(input)).unwrap();
        let response = handler.handle(dbg!(request));
        println!("{}", &serde_json::to_string(&dbg!(response)).unwrap());
    }
}
