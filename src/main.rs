use std::collections::VecDeque;

use tl::{HTMLTag, NodeHandle, ParserOptions};

const URL: &str = "https://enterthegungeon.fandom.com/wiki/Guns";
const TABLE_CLASS: &str = "wikitable";

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).unwrap();

    let client = reqwest::blocking::Client::new();
    let body = client.get(URL).send()?.text()?;

    let dom = tl::parse(&body, ParserOptions::new().track_classes())?;
    let parser = dom.parser();

    let table = dom
        .get_elements_by_class_name("wikitable")
        .next()
        .unwrap()
        .get(parser)
        .unwrap();

    let table_body = table
        .children()
        .unwrap()
        .top()
        .iter()
        .find_map(|node| node.get(parser).unwrap().as_tag())
        .unwrap();

    let mut table_rows = table_body
        .children()
        .top()
        .iter()
        .filter_map(|node| node.get(parser).unwrap().as_tag())
        .collect::<VecDeque<_>>();

    let mut csv_writer = csv::WriterBuilder::new().delimiter(b'|').from_path(path)?;

    let mut print_row = |tag: &HTMLTag<'_>| {
        let children = tag.children();

        let columns = children
            .top()
            .iter()
            .filter_map(|node| node.get(parser).unwrap().as_tag())
            .map(|tag| tag.inner_text(parser).trim().to_owned())
            .skip(1);

        csv_writer.write_record(columns).unwrap();
    };

    let header = table_rows.pop_front().unwrap();

    print_row(header);

    for row in table_rows {
        print_row(row);
    }

    Ok(())
}
