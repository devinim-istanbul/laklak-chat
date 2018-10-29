#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Authenticate {
        token: String
    },

    SendMessage {
        recipient: String,
        message: String
    },

    Ping { },

    Pong { }
}
