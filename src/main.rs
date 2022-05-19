use std::{str, thread, time::Duration};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Opt {
    #[clap(short, long, default_value_t = 15000)]
    delay_in_ms: u64,
    command: Vec<String>,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    // wait for a bit
    thread::sleep(Duration::from_millis(opt.delay_in_ms));

    // execute the command
    let child = std::process::Command::new(&opt.command[0])
        .args(&opt.command[1..])
        .spawn()?;

    let output = child.wait_with_output()?;
    let output = str::from_utf8(output.stdout.as_slice())?;
    println!("{}", output);

    Ok(())
}
