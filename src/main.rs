mod pallet;
mod writer;

use pallet::PalletConfig;

use std::env;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect();
    let model = args[3].parse::<u8>()?;
    let pallet = PalletConfig::from_input(&args[1], model)?;

    writer::write_config(&args[2], &pallet)?;

    Command::new("glpsol")
        .arg("--model")
        .arg(format!("models/model-{}.mod", &args[3]))
        .arg("--data")
        .arg(&args[2])
        .arg("--output")
        .arg("glpsol.out")
        .status()?;
    Ok(())
}
