extern crate goblin;


use goblin::{elf, elf32, elf64};    // ELF Binary
use std::path::Path;
use std::env;


fn is_wanted_sym(sym: &elf::Unified<elf32::sym::Sym, elf64::sym::Sym>) -> bool {
    sym.st_type() != elf::sym::STT_NOTYPE &&
    sym.st_bind() != elf::sym::STB_LOCAL
}

fn main () {
    let bin_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&bin_path);
    // dump
    match elf::Elf::from(&path) {
        Ok(bin) => {
            let mut x = Vec::new();
            for sym in bin.syms
                          .into_iter()
                          .filter(is_wanted_sym) {
                x.push(&bin.strtab[sym.st_name()]);
            }
            x.sort();
            println!("{}", x.join("\n"));
        },
        Err(err) => {
            println!("Not an ELF: {:?}", err);
        }
    }
}
