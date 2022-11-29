mod gun;
mod item;
mod synergy;

use synergy::Synergy;

use std::{collections::HashMap, fs::File};

fn main() {
    let file = std::env::args().nth(1).unwrap();

    let synergies =
        serde_json::from_reader::<_, HashMap<Synergy, synergy::Spec>>(File::open(file).unwrap())
            .unwrap();

    println!("{synergies:#?}");
}
