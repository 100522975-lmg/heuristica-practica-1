mod pallet;

use pallet::Pallet;

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::error::Error;

fn generate_dat(path: &str, pallet: &Pallet) -> io::Result<()> {
    let mut buff = io::BufWriter::new(File::create(path)?); 
    let mut tail: Vec<u8> = Vec::new();

    writeln!(buff, "data;")?;
    writeln!(buff, "param L := {};", pallet.side_length)?;
     
    writeln!(buff, "set BOXES :=")?;
    for (i, p) in pallet.priorities.iter().enumerate() {
        write!(buff, " box{}", i+1)?; 
        writeln!(tail, "  box{} {}", i+1, p)?;
    }
    writeln!(buff, ";\nparam priority :=")?;
    buff.write_all(&tail)?; // append tail buffer to main buffer

    writeln!(buff, ";")?;
    write!(buff, "end;\n")?;
    buff.flush()
}

fn main() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect();
    let pallet = Pallet::from_input(&args[1])?;
   
    generate_dat(&args[2], &pallet)?;
    Ok(())
}
