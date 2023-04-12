use crate::messages::request::RequestBody;
use crate::messages::target::{Node, SeqKv, Target};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreRequest {
    pub src: Node,
    pub dest: SeqKv,
    pub body: StoreRequestBody,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoreRequestBody {
    Read { key: String, msg_id: i64 },
    CAS {},
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreResponse {
    pub src: SeqKv,
    pub dest: Node,
    pub body: StoreResponseBody,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoreResponseBody {
    ReadOk { key: String, msg_id: i64 },
}
