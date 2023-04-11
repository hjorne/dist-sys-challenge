use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Echo {
    msg_id: i64,
    echo: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EchoOk {
    in_reply_to: i64,
    echo: String,
}

impl Echo {
    pub fn reply(self) -> EchoOk {
        EchoOk {
            in_reply_to: self.msg_id,
            echo: self.echo,
        }
    }
}
