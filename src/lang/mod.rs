#[derive(Deserialize)]
/*pub struct Lang {
    pct_change1h: Members,
    pct_change24h: Members,
}*/

struct Members {
    en: String,
    es: String,
    zh_hanz: String,
}

pub enum Lang {
    pct_change1h {
        en: String,
    },
}