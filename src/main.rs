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

fn main() {
    let args = Cli::from_args();
    let pin = match args.out_type {
        OutType::Tone { pin } => pin,
        OutType::Noise { pin } => pin,
    };
    println!(
        "duty={}, out_type={:?}, pin={}",
        args.duty, args.out_type, pin
    );
}
