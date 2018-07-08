// Create the price struct that will
// hold the deserialized response
#[derive(Deserialize)]
pub struct Price {
    pub timestamp: i64,
    pub btc: String,
    pub usd: f64,
    pub eur: f64,
    pub aud: f64,
    pub brl: f64,
    pub cad: f64,
    pub cny: f64,
    pub gbp: f64,
    pub nzd: f64,
    pub dkk: f64,
    pub jpy: f64,
    pub pln: f64,
    pub krw: f64,
    pub rub: f64,
    pub mxn: f64,
    pub sek: f64,
    pub hkd: f64,
    pub myr: f64,
    pub sgd: f64,
    pub chf: f64,
    pub huf: f64,
    pub nok: f64,
    pub thb: f64,
    pub clp: f64,
    pub idr: f64,
    pub try: f64,
    pub ils: f64,
    pub php: f64,
    pub twd: f64,
    pub czk: f64,
    pub inr: f64,
    pub pkr: f64,
    pub zar: f64,
    pub percent_change_1h: PricePercentChange,
    pub percent_change_24h: PricePercentChange,
}

// TODO: Finish adding the rest of the price data
#[derive(Deserialize)]
pub struct PricePercentChange {
    pub btc: String,
    pub usd: String,
    pub eur: String,
    pub aud: String,
    pub brl: String,
    pub cad: String,
    pub cny: String,
    pub gbp: String,
    pub nzd: String,
}

// TODO: write tests
#[cfg(test)]
mod tests {
    use super::*;


}