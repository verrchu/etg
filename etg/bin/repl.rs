use anyhow::Context;

use std::{
    collections::{HashMap, HashSet},
    io::{stdin, stdout, BufRead, Write},
};

use etg::{synergy, Equipable, Gun, Item};

static PROMPT: &[u8] = b"> ";

#[derive(Debug, Default)]
struct Inventory(HashSet<Equipable>);

impl Inventory {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, e: impl Into<Equipable>) {
        self.0.insert(e.into());
    }

    fn remove(&mut self, e: impl Into<Equipable>) {
        self.0.remove(&e.into());
    }

    fn iter(&self) -> impl Iterator<Item = &'_ Equipable> {
        self.0.iter()
    }

    fn guns(&self) -> impl Iterator<Item = &'_ Gun> {
        self.0.iter().filter_map(|e| match e {
            Equipable::Gun(gun) => Some(gun),
            Equipable::Item(_) => None,
        })
    }

    fn items(&self) -> impl Iterator<Item = &'_ Item> {
        self.0.iter().filter_map(|e| match e {
            Equipable::Item(item) => Some(item),
            Equipable::Gun(_) => None,
        })
    }
}

#[derive(Debug)]
struct Synergy {
    tag: synergy::Tag,
    progress: synergy::Progress,
    effect: String,
}

#[derive(Debug)]
struct Progress(Vec<Synergy>);

impl Progress {
    fn new(synergies: impl IntoIterator<Item = Synergy>) -> Self {
        Self(synergies.into_iter().collect())
    }

    fn track_one(&mut self, e: Equipable) {
        self.0.iter_mut().for_each(|synergy| {
            synergy.progress.track(e);
        });
    }

    fn initiated(&self) -> impl Iterator<Item = &'_ Synergy> {
        self.0
            .iter()
            // we rely on an invariant that synergies are sorted by progress
            // completion in descending order and so we stop iterating over
            // them as soon as we observe one with 0 completion
            .take_while(|s| !matches!(s.progress.completion(), (0, _)))
    }

    fn iter(&self) -> impl Iterator<Item = &'_ Synergy> {
        self.0.iter()
    }

    fn sort(&mut self) {
        self.0.sort_by(|a, b| {
            let calc_completion = |synergy: &Synergy| {
                let (n, d) = synergy.progress.completion();
                n as f32 / d as f32
            };

            calc_completion(a).total_cmp(&calc_completion(b)).reverse()
        })
    }
}

fn main() {
    if let Err(err) = run() {
        println!("error: #{err:#}");
    }
}

fn run() -> anyhow::Result<()> {
    let mut stdin = stdin().lock().lines();
    let mut stdout = stdout().lock();

    let mut inventory = Inventory::new();
    let mut progress = init_progress().context("init progress")?;

    let enames = {
        let mut buf = vec![];
        buf.extend_from_slice(Gun::tags());
        buf.extend_from_slice(Item::tags());

        buf
    };

    loop {
        stdout.write_all(PROMPT).context("write prompt")?;
        stdout.flush().context("flush stdout")?;

        let Some(input) = stdin.next() else {
            break;
        };

        let input = input.context("read input")?;

        match input.trim() {
            "inventory" => {
                display_inventory(&inventory, &mut stdout).context("display inventory")?
            }
            "progress" => display_progress(&progress, &mut stdout).context("display progress")?,
            cmd => {
                let parse_tag = |tag| -> Option<Equipable> {
                    if let Some(gun) = Gun::by_tag(tag) {
                        return Some(gun.into());
                    }

                    if let Some(item) = Item::by_tag(tag) {
                        return Some(item.into());
                    }

                    None
                };

                match cmd.split_once(' ') {
                    Some(("+", tag)) => {
                        let Some(e) = parse_tag(tag) else {
                            writeln!(stdout, "unknown tag: {}", tag)?;
                            continue;
                        };

                        inventory.add(e);
                        progress.track_one(e);
                        progress.sort();
                    }
                    Some(("-", tag)) => {
                        let Some(e) = parse_tag(tag) else {
                            writeln!(stdout, "unknown tag: {}", tag)?;
                            continue;
                        };

                        inventory.remove(e);
                        progress =
                            init_progress_from_inventory(&inventory).context("re-init progress")?;
                        progress.sort();
                    }
                    Some(("?", tag)) => {
                        let Some(e) = parse_tag(tag) else {
                            writeln!(stdout, "unknown tag: {}", tag)?;
                            continue;
                        };

                        for synergy in progress.iter() {
                            let (n1, _) = synergy.progress.completion();
                            let mut progress = synergy.progress.clone();
                            progress.track(e);
                            let (n2, d) = progress.completion();

                            if n2 > n1 {
                                writeln!(
                                    stdout,
                                    "- {} ({}/{}) -> ({}/{})",
                                    synergy.tag.tag(),
                                    n1,
                                    d,
                                    n2,
                                    d
                                )?;
                            }
                        }

                        // TODO
                    }
                    _ => todo!(),
                }
            }
        }
    }

    Ok(())
}

// FIXME: reduce duplication
fn display_inventory(inventory: &Inventory, mut writer: impl Write) -> anyhow::Result<()> {
    let mut guns = inventory.guns();
    writer.write_all(b"guns: ")?;
    if let Some(gun) = guns.next() {
        writer.write_all(gun.tag().as_bytes())?;

        for gun in guns {
            writer.write_all(b", ")?;
            writer.write_all(gun.tag().as_bytes())?;
        }
    } else {
        writer.write_all(b"none")?;
    }

    writeln!(writer)?;

    let mut items = inventory.items();
    writer.write_all(b"items: ")?;
    if let Some(item) = items.next() {
        writer.write_all(item.tag().as_bytes())?;

        for item in items {
            writer.write_all(b", ")?;
            writer.write_all(item.tag().as_bytes())?;
        }
    } else {
        writer.write_all(b"none")?;
    }

    writeln!(writer)?;

    writer.flush()?;
    Ok(())
}

fn display_progress(progress: &Progress, mut writer: impl Write) -> anyhow::Result<()> {
    let mut initiated = progress.initiated().peekable();

    writer.write_all(b"synergies:")?;

    if initiated.peek().is_some() {
        for synergy in initiated {
            let (n, d) = synergy.progress.completion();
            let completion = n as f32 / d as f32;

            writeln!(writer)?;
            writer.write_all(b"- ")?;
            writer.write_all(synergy.tag.tag().as_bytes())?;
            writer.write_all(b" - [")?;

            let slots = 20;
            let occupied = (20 as f32 * completion).ceil() as usize;

            for _ in 0..occupied {
                writer.write_all(b"+")?;
            }

            for _ in 0..(slots - occupied) {
                writer.write_all(b" ")?;
            }

            writer.write_all(b"]")?;

            writeln!(writer, " ({}/{})", n, d)?;

            write!(writer, "  parts:")?;

            match &synergy.progress {
                synergy::Progress::Primitive(inner) => {
                    write!(writer, "\n    ")?;
                    display_synergy_progress_primitive(&inner, &mut writer)?;
                }
                synergy::Progress::Combined { left, right } => {
                    write!(writer, "\n    ")?;
                    display_synergy_progress_primitive(&left, &mut writer)?;
                    write!(writer, "\n      and")?;
                    write!(writer, "\n    ")?;
                    display_synergy_progress_primitive(&right, &mut writer)?;
                }
            }

            writeln!(writer, "\n  effect: {}", synergy.effect)?;
        }
    } else {
        writer.write_all(b" none")?;
    }

    writeln!(writer)?;

    writer.flush()?;
    Ok(())
}

fn display_synergy_progress_primitive(
    progress: &synergy::progress::Primitive,
    mut writer: impl Write,
) -> anyhow::Result<()> {
    use synergy::progress::Primitive::*;

    fn write_one(mut writer: impl Write, e: &Equipable, is_complete: bool) -> anyhow::Result<()> {
        let kind = match e {
            Equipable::Gun(_) => "gun",
            Equipable::Item(_) => "item",
        };

        let tag = match e {
            Equipable::Gun(gun) => gun.tag(),
            Equipable::Item(item) => item.tag(),
        };

        let indicator = match is_complete {
            true => "+",
            false => "-",
        };

        write!(writer, "{}{}({})", indicator, kind, tag).map_err(Into::into)
    }

    match progress {
        Single(e, is_complete) => write_one(&mut writer, e, *is_complete)?,
        OneOf(inner) => {
            write!(writer, "one of:")?;
            for (e, is_complete) in inner.iter() {
                write!(writer, " ")?;
                write_one(&mut writer, e, *is_complete)?;
            }
        }
        TwoOf(inner) => {
            write!(writer, "two of:")?;
            for (e, is_complete) in inner.iter() {
                write!(writer, " ")?;
                write_one(&mut writer, e, *is_complete)?;
            }
        }
        AllOf(inner) => {
            write!(writer, "all of:")?;
            for (e, is_complete) in inner.iter() {
                write!(writer, " ")?;
                write_one(&mut writer, e, *is_complete)?;
            }
        }
    }

    Ok(())
}

fn init_progress() -> anyhow::Result<Progress> {
    static SYNERGIES: &str = include_str!("../files/synergies.v3.json");
    let synergies = serde_json::from_str::<HashMap<synergy::Tag, synergy::Spec>>(SYNERGIES)
        .context("parse synergies' definition")?;

    let progress = Progress::new(synergies.into_iter().map(|(tag, spec)| Synergy {
        tag,
        effect: spec.effect,
        progress: spec.parts.into(),
    }));

    Ok(progress)
}

fn init_progress_from_inventory(inventory: &Inventory) -> anyhow::Result<Progress> {
    let mut progress = init_progress()?;

    for e in inventory.iter() {
        progress.track_one(*e);
    }

    Ok(progress)
}
