#[derive(Deserialize, Debug)]
pub struct BlockInfo {
    #[serde(default)]
    pub hash: String,
    #[serde(default)]
    pub height: u64,
    #[serde(default)]
    pub parent_hash: String,
    #[serde(default)]
    pub interlink_hash: String,
    #[serde(default)]
    pub body_hash: String,
    #[serde(default)]
    pub accounts_hash: String,
    #[serde(default)]
    pub pow: String,
    #[serde(default)]
    pub nonce: u64,
    #[serde(default)]
    pub reward: f64,
    #[serde(default)]
    pub miner_address: String,
    #[serde(default)]
    pub difficulty: f64,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub extra_data: String,
    #[serde(default)]
    pub transactions: u64,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub error: String,

}