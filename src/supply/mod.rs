#[derive(Deserialize)]
pub struct Supply {
    pub height: u64,
    pub market_cap: f64,
    pub existing_supply: f64,
    pub circulating_supply: f64,
}