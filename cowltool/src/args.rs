use clap::Parser;

#[derive(Debug, Parser, Default)]
#[clap(name = "cowltool", version = "0.0.1", about = "Cowl's interpeter",)]
pub struct Args {
    /// The bytecode file to read
    pub target: String,
    /// Print the raw bits from the target
    #[clap(long, short, action)]
    pub read_bits: bool,
}

