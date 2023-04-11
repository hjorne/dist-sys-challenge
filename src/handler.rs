use crate::messages::message::Message;
use crate::messages::response::Response;
use crate::state::State;

#[derive(Default)]
pub struct Handler {
    state: State,
}

impl Handler {
    pub fn handle(&mut self, msg: Message) -> Option<Response> {
        match msg {
            Message::Request(req) => Some(req.reply(&mut self.state)),
            Message::Response(_response) => None,
        }
    }
}
