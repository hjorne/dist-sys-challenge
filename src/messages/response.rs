use serde::{Deserialize, Serialize};

use crate::messages::protocols::broadcast::BroadcastOk;
use crate::messages::protocols::echo::EchoOk;
use crate::messages::protocols::generate::GenerateOk;
use crate::messages::protocols::init::InitOk;
use crate::messages::protocols::sync_broadcast::SyncBroadcastOk;
use crate::messages::protocols::read::ReadOk;
use crate::messages::protocols::topology::TopologyOk;
use crate::messages::target::Target;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub src: Target,
    pub dest: Target,
    pub body: ResponseBody,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseBody {
    InitOk(InitOk),
    EchoOk(EchoOk),
    GenerateOk(GenerateOk),
    BroadcastOk(BroadcastOk),
    ReadOk(ReadOk),
    TopologyOk(TopologyOk),
    SyncBroadcastOk(SyncBroadcastOk),
}
