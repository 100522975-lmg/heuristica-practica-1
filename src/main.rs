use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

struct Pallet {
    side_length: i32,
    priorities: Vec<i32>,
}

fn parse_input(path: &str) -> Pallet {
    let contents = fs::read_to_string(path)
        .expect("Could not read the file.")
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    return Pallet {
        side_length: contents[0]
            .parse::<i32>()
            .unwrap(),
        priorities: contents[1]
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect(),
    };
}

fn generate_dat(path: &str, pallet: &Pallet) -> io::Result<()> {
    let mut buff = io::BufWriter::new(File::create(path)?); 
    writeln!(buff, "data;")?;
    writeln!(buff, "param L := {}", pallet.side_length)?;
    
    writeln!(buff, "set BOXES :=")?;
    for (i, _) in pallet.priorities.iter().enumerate() {
        write!(buff, " box{}", i+1)?; 
    }
    
    writeln!(buff, "\nparam priority :=")?;
    for (i, p) in pallet.priorities.iter().enumerate() {
        write!(buff, " box{} {}", i+1, p)?;
    }
    writeln!(buff, ";")?;
    write!(buff, "end;\n")?;
    buff.flush()
}

fn main() -> io::Result<()> { 
    let args: Vec<String> = env::args().collect();
    let pallet = parse_input(&args[1]);
   
    generate_dat(&args[2], &pallet)?;
    Ok(())
}
