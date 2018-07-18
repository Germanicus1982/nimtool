#[derive(Deserialize, Debug)]
pub struct Hashrate {
    #[serde(default)]
    pub timestamp: u64,
    #[serde(default)]
    pub hashrate: f64,
}