#[derive(Deserialize, Debug)]
pub struct AddressBook {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub error: String,
}