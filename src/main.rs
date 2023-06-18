use clap::Parser;

mod generator;
use generator::{AddressGenerator, TestNet1Generator, TestNet2Generator, TestNet3Generator};

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = false,
    disable_help_subcommand = true
)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "Number of generate addresses"
    )]
    number: u8,

    #[arg(short, long, default_value_t = 1, help = "Network block")]
    block: u8,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let generator: Box<dyn AddressGenerator> = match &args.block {
        1 => Box::new(TestNet1Generator {}),
        2 => Box::new(TestNet2Generator {}),
        3 => Box::new(TestNet3Generator {}),
        _ => {
            println!("block None");
            return;
        }
    };

    generator.generate();
}
