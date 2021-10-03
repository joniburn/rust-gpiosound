use std::io::{stdin, Result};
use std::os::unix::io::AsRawFd;

use mio::unix::SourceFd;
use mio::{Poll, Token, Interest};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "50")]
    duty: u8,
    #[structopt(subcommand)]
    out_type: OutType,
}

#[derive(Debug, StructOpt)]
enum OutType {
    Tone { pin: u8 },
    Noise { pin: u8 },
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let pin = match args.out_type {
        OutType::Tone { pin } => pin,
        OutType::Noise { pin } => pin,
    };
    println!(
        "duty={}, out_type={:?}, pin={}",
        args.duty, args.out_type, pin
    );

    // let mut stdin_fd = io::stdin().i

    let poll = Poll::new()?;

    let token_stdin = Token(0);
    poll.registry()
        .register(&mut SourceFd(&stdin().as_raw_fd()), token_stdin, Interest::READABLE)?;


    // TODO
    Ok(())
}
