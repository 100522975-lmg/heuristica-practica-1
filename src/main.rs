mod pallet;
mod writer;
mod output;
mod drawer;

use pallet::PalletConfig;
use drawer::Drawer;

use std::env;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> { 
    let args: Vec<String> = env::args().collect();
    let model = args[3].parse::<u8>()?;
    let pallet = PalletConfig::from_input(&args[1], model)?;

    writer::write_config(&args[2], &pallet)?;

    let out = Command::new("glpsol")
        .arg("--model")
        .arg(format!("models/model-{}.mod", &args[3]))
        .arg("--data")
        .arg(&args[2])
        .arg("--output")
        .arg("/dev/stderr")
        .output()?;
    
    let stderr = String::from_utf8_lossy(&out.stderr));

    output::format_header(&stderr);

    let mut drawer = Drawer::new(&pallet , pallet.p.len());
    drawer.parse(&stderr);
}
