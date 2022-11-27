use std::collections::{HashSet, VecDeque};

use tl::{HTMLTag, NodeHandle, ParserOptions};

const URL: &str = "https://enterthegungeon.fandom.com/wiki/Synergies";
const TABLE_CLASS: &str = "wikitable";

fn main() -> anyhow::Result<()> {
    let path1 = std::env::args().nth(1).unwrap();
    let path2 = std::env::args().nth(2).unwrap();

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

    let mut csv1 = csv::WriterBuilder::new().delimiter(b'|').from_path(path1)?;
    let mut csv2 = csv::WriterBuilder::new().delimiter(b'|').from_path(path2)?;

    let mut print_row = |tag: &HTMLTag<'_>| {
        let children = tag.children();
        let mut children = children
            .top()
            .iter()
            .filter_map(|node| node.get(parser).unwrap().as_tag())
            .collect::<Vec<_>>();

        let last = children.last().unwrap();
        let last_children = last
            .children()
            .top()
            .iter()
            .map(|node| node.get(parser).unwrap())
            .collect::<Vec<_>>();

        if last_children.len() == 2 {
            if let Some(new_line) = last_children[1].as_raw() {
                if new_line.as_utf8_str().trim() == "" {
                    children.pop();
                }
            }
        }

        let name = children[0].inner_text(parser).trim().to_string();
        let effect = children
            .last()
            .unwrap()
            .inner_text(parser)
            .trim()
            .to_string();

        if name == "Phoenix Up" {
            children.pop();
        }

        let clen = children.len();

        let get_refs = |tag: &HTMLTag<'_>| {
            let children = tag.children().all(parser);

            children
                .iter()
                .filter_map(|node| node.as_tag())
                .filter(|node| node.name().as_utf8_str() == "a")
                .map(|r| {
                    r.attributes()
                        .get("title")
                        .unwrap()
                        .unwrap()
                        .as_utf8_str()
                        .to_string()
                })
                .collect::<HashSet<_>>()
        };

        if clen == 3 {
            let col = children[1];
            let parts = Vec::from_iter(get_refs(col)).join(",");

            csv1.write_record(&[&name, &parts, &effect]).unwrap();
        }

        if clen == 4 {
            let col_left = children[1];
            let parts_left = Vec::from_iter(get_refs(col_left)).join(",");
            let col_right = children[2];
            let parts_right = Vec::from_iter(get_refs(col_right)).join(",");

            csv2.write_record(&[&name, &parts_left, &parts_right, &effect])
                .unwrap();
        }
    };

    let header = table_rows.pop_front().unwrap();

    print_row(header);

    for row in table_rows {
        print_row(row);
    }

    Ok(())
}
