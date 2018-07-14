#[derive(Deserialize, Debug)]
pub struct NetStat {
    pub hashrate: f64,
    pub last_found: u64,
    pub height: u64,
    pub difficulty: f64,
    pub last_reward: f64,
    pub nim_day_kh: String,
}