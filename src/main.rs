mod process;
mod syscalls;
mod tracer;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tracer")]
#[command(about = "Toy system call tracer", long_about = None)]
struct Args {
    cmd: String,
    args: Vec<String>,
}

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    let Args { cmd, args } = Args::parse();

    process::spawn_tracee(&cmd, &args)?;
    Ok(())
}
