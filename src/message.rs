use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub src: Target,
    pub dest: Target,
    pub body: RequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub src: Target,
    pub dest: Target,
    pub body: ResponseBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestBody {
    Init {
        msg_id: i64,
        node_id: Node,
        node_ids: Vec<Node>,
    },
    Echo {
        msg_id: i64,
        echo: String,
    },
    Generate {
        msg_id: i64,
    },
    Broadcast {
        msg_id: i64,
        message: i64,
    },
    Read {
        msg_id: i64,
    },
    Topology {
        msg_id: i64,
        topology: HashMap<Node, Vec<Node>>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseBody {
    InitOk {
        in_reply_to: i64,
    },
    EchoOk {
        in_reply_to: i64,
        echo: String,
    },
    GenerateOk {
        in_reply_to: i64,
        id: String,
    },
    BroadcastOk {
        in_reply_to: i64,
    },
    ReadOk {
        in_reply_to: i64,
        messages: Vec<i64>,
    },
    TopologyOk {
        in_reply_to: i64,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Target {
    Client(Client),
    Node(Node),
}

#[derive(Debug, Copy, Clone)]
pub struct Client(pub i64);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Node(pub i64);

impl<'de> Deserialize<'de> for Client {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut chars = s.chars();
        match chars.next().unwrap() {
            'c' => Ok(Client(chars.as_str().parse().unwrap())),
            _ => Err(Error::custom(format!("Bad target type {}", s))),
        }
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut chars = s.chars();
        match chars.next().unwrap() {
            'n' => Ok(Node(chars.as_str().parse().unwrap())),
            _ => Err(Error::custom(format!("Bad target type {}", s))),
        }
    }
}

impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("c{}", self.0))
    }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("n{}", self.0))
    }
}
