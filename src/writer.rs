use pallet::Pallet;
use std::io::{self, Write};
use std::fs::{self, File};

pub fn generate_dat(path: &str, pallet: &Pallet, model: i8) -> io::Result<()> {
    let mut out = io::BufWriter::new(File::create(path)?); 
    let mut tail: Vec<u8> = Vec::new();

    writeln!(out, "data;")?;
    writeln!(out, "param L := {};", pallet.side_length)?;

    if model > 2 {
        writeln!(out, "param H := {};", pallet.height)?;
    }
    writeln!(out, "set BOXES :=")?;
    for (i, p) in pallet.priorities.iter().enumerate() {
        write!(out, " box{}", i+1)?; 
        writeln!(tail, "  box{} {}", i+1, p)?;
    }
    writeln!(out, ";\nparam priority :=")?;
    out.write_all(&tail)?; // append tail outer to main outer

    writeln!(out, ";")?;
    write!(out, "end;\n")?;
    out.flush()
}
