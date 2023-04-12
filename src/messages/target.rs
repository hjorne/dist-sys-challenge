use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Target {
    Client(Client),
    Node(Node),
    SeqKv(SeqKv),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Client(pub i64);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct Node(pub i64);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "seq-kv")]
pub struct SeqKv;

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

#[cfg(test)]
mod tests {
    use crate::messages::target::{Client, Node, Target};

    #[test]
    fn client_serdes() {
        let json = "\"c20\"";
        assert_eq!(serde_json::to_string(&Client(20)).unwrap(), json);
        assert_eq!(serde_json::from_str::<Client>(json).unwrap(), Client(20));
    }

    #[test]
    fn node_serdes() {
        let json = "\"n20\"";
        assert_eq!(serde_json::to_string(&Node(20)).unwrap(), json);
        assert_eq!(serde_json::from_str::<Node>(json).unwrap(), Node(20));
    }

    #[test]
    fn target_node_serdes() {
        let json = "\"n20\"";

        assert_eq!(
            serde_json::to_string(&Target::Node(Node(20))).unwrap(),
            json
        );
        assert_eq!(
            serde_json::from_str::<Target>(json).unwrap(),
            Target::Node(Node(20))
        );
    }

    #[test]
    fn target_client_serdes() {
        let json = "\"c20\"";

        assert_eq!(
            serde_json::to_string(&Target::Client(Client(20))).unwrap(),
            json
        );
        assert_eq!(
            serde_json::from_str::<Target>(json).unwrap(),
            Target::Client(Client(20))
        );
    }
}
