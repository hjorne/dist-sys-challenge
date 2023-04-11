use std::sync::mpsc::Sender;

use crate::messages::request::Request;
use crate::messages::response::{Response, ResponseBody};
use crate::state::State;
use crate::syncer::SyncMsg;

pub struct Handler {
    state: State,
}

impl Handler {
    pub fn new(sender: Sender<SyncMsg>) -> Handler {
        Handler {
            state: State::new(sender),
        }
    }

    pub fn handle_request(&mut self, request: Request) -> Response {
        request.reply(&mut self.state)
    }

    pub fn handle_response(&mut self, response: Response) {
        if let ResponseBody::SyncBroadcastOk(sync_broadcast_ok) = response.body {
            self.state
                .sender
                .send(SyncMsg::Syncd {
                    msg_id: sync_broadcast_ok.in_reply_to,
                })
                .unwrap();
        }
    }
}
