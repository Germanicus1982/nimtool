#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(default)]
    pub hash: String,
    #[serde(default)]
    pub height: u64,
    #[serde(default)]
    pub block_hash: String,
    #[serde(default)]
    pub transaction_index: u64,
    #[serde(default)]
    pub from_address: String,
    #[serde(default)]
    pub to_address: String,
    #[serde(default)]
    pub value: f64,
    #[serde(default)]
    pub fee: f64,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub error: String,
}