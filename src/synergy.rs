use super::gun::Gun;
use super::item::Item;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum Part {
    Gun(Gun),
    Item(Item),
}

#[derive(Debug, Serialize, Deserialize)]
struct Synergy {
    name: String,
    effect: String,
    parts: Parts,
}

#[derive(Debug, Serialize, Deserialize)]
enum Parts {
    Single(Part),
    OneOf(Vec<Part>),
    TwoOf(Vec<Part>),
    AllOf(Vec<Part>),
    Combined(Box<Parts>, Box<Parts>)
}
