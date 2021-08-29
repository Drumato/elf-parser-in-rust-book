use super::{ElfHeader64, Section64};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Elf64 {
    pub header: ElfHeader64,
    pub sections: Vec<Section64>,
}
