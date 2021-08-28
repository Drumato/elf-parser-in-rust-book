use io::prelude::*;
use std::io;

mod elf;
mod parser;

fn main() -> io::Result<()> {
    use std::env;
    use std::fs;
    use std::process;

    let file_path = env::args().nth(1);
    if file_path.is_none() {
        eprintln!("usage: ./main <elf-file-path>");
        process::exit(1);
    }

    let mut file = fs::File::open(file_path.unwrap())?;
    let mut raw_elf = Vec::new();
    file.read_to_end(&mut raw_elf)?;

    let elf_repr = parser::parse_64bit_elf_header_64bit(&raw_elf);
    if elf_repr.is_err() {
        eprintln!("failed to parse elf; err => {}", elf_repr.err().unwrap());
        process::exit(1);
    }

    dbg!(elf_repr.unwrap().1);

    Ok(())
}
