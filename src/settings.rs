#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub udp: Udp,
    pub http: Http,
}

#[derive(serde::Deserialize, Clone)]
pub struct Udp {
    pub address: String,
    pub port: u32,
}

#[derive(serde::Deserialize, Clone)]
pub struct Http {
    pub address: String,
    pub port: u32,
}

pub fn get_settings() -> Settings {
    toml::from_str(
        &std::fs::read_to_string("bummer.toml").expect("Failure to read bummer.toml")
    ).expect("Failure to parse bummer.toml into Settings")
}
