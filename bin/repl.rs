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

    fn iniitated(&self) -> impl Iterator<Item = &'_ Synergy> {
        self.0
            .iter()
            // we rely on an invariant that synergies are sorted by progress
            // completion in descending order and so we stop iterating over
            // them as soon as we observe one with 0 completion
            .take_while(|s| !matches!(s.progress.completion(), (0, _)))
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

    loop {
        stdout.write_all(PROMPT).context("write prompt")?;
        stdout.flush().context("flush stdout")?;

        let Some(input) = stdin.next() else {
            break;
        };

        let input = input.context("read input")?;
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
