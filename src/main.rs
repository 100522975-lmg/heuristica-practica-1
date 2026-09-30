use std::env;
use std::fs;

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

fn main() { 
    let args: Vec<String> = env::args().collect();
    let pallet = parse_input(&args[1]);

    println!("{}", pallet.side_length);
    println!("{:?}", pallet.priorities);

}
