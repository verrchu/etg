use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Kind {
    Automatic,
    Beam,
    Burst,
    Charged,
    Semiautomatic,
    Type,
    Varies,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Essential {
    name: String,
    notes: Option<String>,
    tier: Option<Tier>,
    #[serde(rename(deserialize = "type"))]
    kind: Kind,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct V1 {
    name: String,
    notes: Option<String>,
    quote: Option<String>,
    tier: Option<Tier>,
    #[serde(rename = "type")]
    kind: Kind,
    dps: Option<String>,
    magazine_size: Option<String>,
    ammo_capacity: Option<String>,
    damage: Option<String>,
    fire_rate: Option<String>,
    reload_time: Option<String>,
    shot_speed: Option<String>,
    range: Option<String>,
    force: Option<String>,
    spread: Option<String>,
}
