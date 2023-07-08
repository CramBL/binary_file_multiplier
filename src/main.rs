use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "binmult - Copy and append content from raw data files without mutating any contents"
)]
#[command(bin_name = "binmult", version)]
#[command(author = "Marc König <mbkj@tutamail.com>")]
#[command(
    about = "binmult takes a file as an input and creates a new file with as many appended copies of the input file as requested"
)]
#[command(
    long_about = "\nbinmult takes a file as an input and creates a new file with as many appended copies of the input file as requested.\n\
useful for my need of large files for benchmarking in CI pipelines without downloading external content.\n\
\n\
Project home page: TBD"
)]
pub struct Cfg {
    /// Input file.
    #[arg(name = "Input data", required = true)]
    file_in: PathBuf,

    /// Output file.
    #[arg(
        name = "Output Data",
        short = 'o',
        long = "output",
        visible_alias = "out",
        required = true
    )]
    file_out: PathBuf,
}

fn read_binary_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_binary_file(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Cfg::parse();

    let input_path = config.file_in;
    let output_path = config.file_out;

    let contents = read_binary_file(input_path.to_str().unwrap())?;

    let size = contents.len();

    // How many to make 100 MB?
    let duplication_factor = 1024 * 1024 * 100 / size;
    let mut bytes_100_mb: Vec<u8> = std::vec::Vec::with_capacity(1024 * 1024 * 100);

    for _ in 0..duplication_factor {
        bytes_100_mb.extend_from_slice(&contents);
    }

    write_binary_file(output_path.to_str().unwrap(), &bytes_100_mb)?;

    Ok(())
}
