use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: Target,
    pub dest: Target,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "type")]
    pub type_field: Type,
    pub msg_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<i64>,
    pub echo: String,
}

#[derive(Debug)]
pub enum Target {
    Client(i64),
    Node(i64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Echo,
    EchoOk
}

impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut chars = s.chars();
        match chars.next().unwrap() {
            'c' => Ok(Target::Client(chars.as_str().parse().unwrap())),
            'n' => Ok(Target::Node(chars.as_str().parse().unwrap())),
            _ => Err(D::Error::custom(format!("Bad target type {}", s)))
        }
    }
}

impl Serialize for Target {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Target::Client(n) => serializer.serialize_str(&format!("c{}", n)),
            Target::Node(n) => serializer.serialize_str(&format!("n{}", n))
        }
    }
}


fn main() {
    println!("Hello, world!");
    let body = r#"{
                      "src": "c1",
                      "dest": "n1",
                      "body": {
                        "type": "echo_ok",
                        "msg_id": 1,
                        "echo": "Please echo 35"
                      }
                    }"#;
    let r: Message = serde_json::from_str(body).unwrap();
    dbg!(&r);
    dbg!(&serde_json::to_string(&r).unwrap());
}
