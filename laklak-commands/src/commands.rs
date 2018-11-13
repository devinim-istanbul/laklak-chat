use codec;

pub trait Command {
    fn encode(&self) -> String;
}

#[derive(Debug)]
pub struct SendMessage {
    recipient: String,
    message: String,
}

#[derive(Debug)]
pub struct Authenticate {
    token: String,
}

#[derive(Debug)]
pub struct Ping {}

#[derive(Debug)]
pub struct Pong {}

impl SendMessage {
    pub fn new(recipient: String, message: String) -> Self {
        SendMessage { recipient: recipient, message: message }
    }
}

impl Authenticate {
    pub fn new(token: String) -> Self {
        Authenticate { token: id_like!(token) }
    }
}

impl Ping {
    pub fn new() -> Self {
        Ping {}
    }
}

impl Pong {
    pub fn new() -> Self {
        Pong {}
    }
}

impl Command for SendMessage {
    fn encode(&self) -> String {
        codec::encode_base64(&format!("SEND|{}|{}", self.recipient, self.message))
    }
}

impl Command for Authenticate {
    fn encode(&self) -> String {
        codec::encode_base64(&format!("AUTH|{}|", self.token))
    }
}
impl Command for Ping {
    fn encode(&self) -> String {
        codec::encode_base64(&format!("PING"))
    }
}
impl Command for Pong {
    fn encode(&self) -> String {
        codec::encode_base64(&format!("PONG"))
    }
}

// I prefer this to be some sort of
// String -> () function that spawns
// the required Actor itself.
pub fn create_command<'a>(s: String) -> Box<Command> {
    let parts: Vec<&str> = s.split("|").collect();
    match parts[0] {
        "SEND" => Box::new(SendMessage::new(String::from(parts[1]), String::from(parts[2]))),
        "AUTH" => Box::new(Authenticate::new(String::from(parts[1]))),
        "PING" => Box::new(Ping::new()),
        "PONG" => Box::new(Pong::new()),
        unknown_command => panic!("Unknown command: {}", unknown_command),
    }
}
