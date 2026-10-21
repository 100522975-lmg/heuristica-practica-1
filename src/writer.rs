use pallet::{PalletConfig, PalletModel};
use std::io::{self, Write};
use std::fs::File;

fn write_boxes<W: Write>(out: &mut W, set: &str, n: usize) -> io::Result<()> {
    write!(out, "\nset {set} :=")?;
    for i in 1..=n {
        write!(out, "\n  box{i}")?;
    }
    writeln!(out, ";")
}

fn write_values<W: Write>(out: &mut W, name: &str, values: &[i32]) -> io::Result<()> {
    write!(out, "\nparam {name} :=")?;
    for (i, value) in values.iter().enumerate() {
        write!(out, "\n  box{} {}", i + 1, value)?;
    }
    writeln!(out, ";")
}

fn write_common<W: Write>(out: &mut W, p: &Vec<i32>, w: &Vec<i32>, c: &Vec<i32>, n: usize) -> io::Result<()> {
    write_boxes(out, "BOXES", n)?;
    write_values(out, "priority", p)?;
    write_values(out, "weight", w)?;
    write_values(out, "capacity", c)?;
    Ok(())
}

pub fn write_config(path: &str, pallet: &PalletConfig) -> io::Result<()> {
    let mut out = io::BufWriter::new(File::create(path)?);
    let n = pallet.p.len();
    
    writeln!(out, "data;")?;
    writeln!(out, "param L := {};", pallet.l)?;

    match &pallet.model {
        PalletModel::Model1 => {
            write_boxes(&mut out, "BOXES", n)?;
            write_values(&mut out, "priority", &pallet.p)?;
        },
        PalletModel::Model2 { h, w, c } => {
            writeln!(out, "param H := {h};")?;
            write_common(&mut out, &pallet.p, w, c, n)?;
        },
        PalletModel::Model3 { h, w, c } => {
            writeln!(out, "param H := {h};")?;
            writeln!(out, "param n := {n};")?;
            writeln!(out, "param M := 9999;")?;
            write_common(&mut out, &pallet.p, w, c, n)?;
        },
        PalletModel::Model4 { h, k, w, c } => {
            writeln!(out, "param H := {h};")?;
            writeln!(out, "param n := {n};")?;
            writeln!(out, "param M := 9999;")?;
            writeln!(out, "param p_cost := {k};")?;
            write_common(&mut out, &pallet.p, w, c, n)?;
        },
    }

    writeln!(out, "end;")?;
    out.flush()
}
