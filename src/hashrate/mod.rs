#[derive(Deserialize, Debug)]
pub struct Hashrate {
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub hashrate: f64,
}