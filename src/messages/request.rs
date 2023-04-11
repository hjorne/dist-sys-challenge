use serde::{Deserialize, Serialize};

use crate::messages::protocols::broadcast::Broadcast;
use crate::messages::protocols::echo::Echo;
use crate::messages::protocols::generate::Generate;
use crate::messages::protocols::init::Init;
use crate::messages::protocols::read::Read;
use crate::messages::protocols::topology::Topology;
use crate::messages::response::{Response, ResponseBody};
use crate::messages::target::Target;
use crate::state::State;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub src: Target,
    pub dest: Target,
    pub body: RequestBody,
}

impl Request {
    pub fn reply(self, state: &mut State) -> Response {
        Response {
            src: self.dest,
            dest: self.src,
            body: self.body.reply(state),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestBody {
    Init(Init),
    Echo(Echo),
    Generate(Generate),
    Broadcast(Broadcast),
    Read(Read),
    Topology(Topology),
}

impl RequestBody {
    fn reply(self, state: &mut State) -> ResponseBody {
        match self {
            RequestBody::Init(init) => ResponseBody::InitOk(init.reply(state)),
            RequestBody::Echo(echo) => ResponseBody::EchoOk(echo.reply()),
            RequestBody::Generate(generate) => ResponseBody::GenerateOk(generate.reply()),
            RequestBody::Broadcast(broadcast) => ResponseBody::BroadcastOk(broadcast.reply(state)),
            RequestBody::Read(read) => ResponseBody::ReadOk(read.reply(state)),
            RequestBody::Topology(topology) => ResponseBody::TopologyOk(topology.reply(state)),
        }
    }
}
