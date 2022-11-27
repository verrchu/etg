mod gun;
mod item;
mod synergy;

use std::fs::File;
use std::io::Write;

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let input = File::open(input).unwrap();

    let output = std::env::args().nth(2).unwrap();
    let mut output = File::create(output).unwrap();

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .from_reader(input);

    let records = csv_reader
        .deserialize()
        .collect::<Result<Vec<item::Essential>, _>>()
        .unwrap();
    let records = ron::ser::to_string_pretty(&records, Default::default()).unwrap();

    writeln!(&mut output, "{}", records).unwrap();
}
