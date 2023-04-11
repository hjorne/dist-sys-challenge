use crate::messages::request::Request;
use crate::messages::response::{Response, ResponseBody};
use crate::state::State;

#[derive(Default)]
pub struct Handler {
    state: State,
}

impl Handler {
    pub fn handle_request(&mut self, request: Request) -> Response {
        request.reply(&mut self.state)
    }
    pub fn handle_response(&mut self, response: Response) {
        if let ResponseBody::BroadcastOk(broadcast_ok) = response.body {
            self.state.syncer.receive(broadcast_ok.in_reply_to);
        }
    }
}
