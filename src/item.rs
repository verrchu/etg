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
    Active,
    Passive,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Essential {
    name: String,
    effect: String,
    tier: Option<Tier>,
    #[serde(rename(deserialize = "type"))]
    kind: Kind,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct V1 {
    name: String,
    #[serde(rename(deserialize = "type"))]
    kind: Kind,
    quote: String,
    tier: Option<Tier>,
    effect: String,
}
