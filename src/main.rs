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

    let elf_repr = parser::parse_64bit_elf(&raw_elf);
    if elf_repr.is_err() {
        eprintln!("failed to parse elf; err => {}", elf_repr.err().unwrap());
        process::exit(1);
    }

    let elf_file = elf_repr.unwrap().1;
    {
        println!("ELF Header:");
        println!("\tClass: {:?}", elf_file.header.id.class);
        println!("\tData: {:?}", elf_file.header.id.data);
        println!("\tEntry: 0x{:x}", elf_file.header.entry);
        println!("\tPHT Info:");
        println!("\t\tOffset: {:?}", elf_file.header.pht_offset);
        println!("\t\tEntries: {:?}", elf_file.header.pht_entries);
        println!("\t\tEntry Size: 0x{:x}", elf_file.header.pht_entry_size);
        println!("\tSHT Info:");
        println!("\t\tOffset: 0x{:x}", elf_file.header.sht_offset);
        println!("\t\tEntries: {}", elf_file.header.sht_entries);
        println!("\t\tEntry Size: 0x{:x}", elf_file.header.sht_entry_size);
        println!(
            "\t\tName Table Index: {:?}",
            elf_file.header.sht_string_table_index
        );
    }

    {
        for (i, sct) in elf_file.sections.iter().enumerate() {
            println!("Sections[{}]:", i);
            println!("\tName: {}", sct.name);
            println!("\tType: {:?}", sct.header.ty);
            println!("\tSize: 0x{:x}", sct.header.size);
            println!("\tEntry Size: 0x{:x}", sct.header.entry_size);
        }
    }

    Ok(())
}
