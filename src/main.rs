use std::fs::File;
use std::io::{self, Read, Write};

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

fn main() {
    let input_path = r"..\fastpasta\tests\test-data\10_rdh.raw";
    let output_path = r"..\fastpasta\tests\test-data\tmp\tmp_10_rdh.raw";

    match read_binary_file(input_path) {
        Ok(bytes) => {
            println!("Read {} bytes from the file.", bytes.len());

            let size = bytes.len();

            // How many to make 100 MB?
            let duplication_factor = 1024 * 1024 * 100 / size;

            println!("Duplicates required for 100 MB = {} x", duplication_factor);

            let mut bytes_100_mb: Vec<u8> = std::vec::Vec::with_capacity(1024 * 1024 * 100);

            for _ in 0..duplication_factor {
                bytes_100_mb.extend_from_slice(&bytes);
            }

            match write_binary_file(output_path, &bytes_100_mb) {
                Ok(_) => {
                    println!(
                        "Successfully wrote {} bytes to the output file.",
                        bytes_100_mb.len()
                    );
                }
                Err(err) => {
                    eprintln!("Error writing file: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
