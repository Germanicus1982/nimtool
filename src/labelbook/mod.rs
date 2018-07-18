#[derive(Deserialize, Debug)]
pub struct LabelBook {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub error: String,
}