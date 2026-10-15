mod pallet;
mod writer;

use pallet::Pallet;

use std::env;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect();
    let pallet = Pallet::from_input(&args[1])?;
    let model = args[3].parse::<i8>()?;

    writer::generate_dat(&args[2], &pallet, model)?;
    
    let output = Command::new("glpsol")
        .arg("--model")
        .arg(format!("models/model-{}.mod", &args[3]))
        .arg("--data")
        .arg(format!("{}", &args[2]))
        .status()?;
    Ok(())
}
