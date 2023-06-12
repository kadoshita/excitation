use clap::{Parser};

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
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
