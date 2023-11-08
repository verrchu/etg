mod gun;
mod item;
mod synergy;

use synergy::Synergy;

use std::{collections::HashMap, fs::File};

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let synergies = serde_json::from_reader::<_, HashMap<synergy::Tag, synergy::Spec>>(
        File::open(input).unwrap(),
    )
    .unwrap();

    println!("{synergies:?}");
}
