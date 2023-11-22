use anyhow::Context;

use std::io::{stdin, stdout, BufRead, Write};

static PROMPT: &[u8] = b"> ";

fn main() {
    if let Err(err) = run() {
        println!("error: #{err:#}");
    }
}

fn run() -> anyhow::Result<()> {
    let mut stdin = stdin().lock().lines();
    let mut stdout = stdout().lock();

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
