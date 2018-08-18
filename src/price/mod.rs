#[derive(Deserialize, Debug)]
pub struct Price {
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub btc: String,
    #[serde(default)]
    pub usd: f64,
    #[serde(default)]
    pub eur: f64,
    #[serde(default)]
    pub aud: f64,
    #[serde(default)]
    pub brl: f64,
    #[serde(default)]
    pub cad: f64,
    #[serde(default)]
    pub cny: f64,
    #[serde(default)]
    pub gbp: f64,
    #[serde(default)]
    pub nzd: f64,
    #[serde(default)]
    pub dkk: f64,
    #[serde(default)]
    pub jpy: f64,
    #[serde(default)]
    pub pln: f64,
    #[serde(default)]
    pub krw: f64,
    #[serde(default)]
    pub rub: f64,
    #[serde(default)]
    pub mxn: f64,
    #[serde(default)]
    pub sek: f64,
    #[serde(default)]
    pub hkd: f64,
    #[serde(default)]
    pub myr: f64,
    #[serde(default)]
    pub sgd: f64,
    #[serde(default)]
    pub chf: f64,
    #[serde(default)]
    pub huf: f64,
    #[serde(default)]
    pub nok: f64,
    #[serde(default)]
    pub thb: f64,
    #[serde(default)]
    pub clp: f64,
    #[serde(default)]
    pub idr: f64,
    #[serde(default)]
    pub try: f64,
    #[serde(default)]
    pub ils: f64,
    #[serde(default)]
    pub php: f64,
    #[serde(default)]
    pub twd: f64,
    #[serde(default)]
    pub czk: f64,
    #[serde(default)]
    pub inr: f64,
    #[serde(default)]
    pub pkr: f64,
    #[serde(default)]
    pub zar: f64,
    pub percent_change_1h: PricePercentChange,
    pub percent_change_24h: PricePercentChange,
}

#[derive(Deserialize, Debug)]
pub struct PricePercentChange {
    #[serde(default)]
    pub btc: String,
    #[serde(default)]
    pub usd: String,
    #[serde(default)]
    pub eur: String,
    #[serde(default)]
    pub aud: String,
    #[serde(default)]
    pub brl: String,
    #[serde(default)]
    pub cad: String,
    #[serde(default)]
    pub cny: String,
    #[serde(default)]
    pub gbp: String,
    #[serde(default)]
    pub nzd: String,
    #[serde(default)]
    pub dkk: String,
    #[serde(default)]
    pub jpy: String,
    #[serde(default)]
    pub pln: String,
    #[serde(default)]
    pub krw: String,
    #[serde(default)]
    pub rub: String,
    #[serde(default)]
    pub mxn: String,
    #[serde(default)]
    pub sek: String,
    #[serde(default)]
    pub hkd: String,
    #[serde(default)]
    pub myr: String,
    #[serde(default)]
    pub sgd: String,
    #[serde(default)]
    pub chf: String,
    #[serde(default)]
    pub huf: String,
    #[serde(default)]
    pub nok: String,
    #[serde(default)]
    pub thb: String,
    #[serde(default)]
    pub clp: String,
    #[serde(default)]
    pub idr: String,
    #[serde(default)]
    pub try: String,
    #[serde(default)]
    pub ils: String,
    #[serde(default)]
    pub php: String,
    #[serde(default)]
    pub twd: String,
    #[serde(default)]
    pub czk: String,
    #[serde(default)]
    pub inr: String,
    #[serde(default)]
    pub pkr: String,
    #[serde(default)]
    pub zar: String,

}

#[derive(Deserialize, Debug)]
pub struct PriceDay {
    #[serde(default)]
    pub timestamp: u64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub btc: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub usd: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub eur: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub aud: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub brl: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub cad: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub cny: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gbp: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub nzd: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dkk: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub jpy: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pln: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub krw: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub rub: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mxn: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sek: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hkd: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub myr: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sgd: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub chf: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub huf: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub nok: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thb: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub clp: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub idr: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub try: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub ils: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub php: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub twd: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub czk: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub inr: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pkr: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub zar: Option<f64>,

}