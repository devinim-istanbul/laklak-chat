#[derive(Debug, Deserialize)]
pub struct HTTP {
    pub hostname: String,
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct TCP {
    pub hostname: String,
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub http: HTTP,
    pub tcp: TCP
}
