use std::fs;
use std::error::Error;

pub struct Pallet {
    pub side_length: i32,
    pub priorities: Vec<i32>,
    pub cost: Option<i32>,
    pub weights: Option<Vec<i32>>,
    pub capacities: Option<Vec<i32>>,
}

impl Pallet {
    pub fn from_input(path: &str) -> Result<Pallet, Box<dyn Error>> {
        let contents: Vec<Vec<i32>> = fs::read_to_string(path)?
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|x| x.parse::<i32>())
                    .collect::<Result<Vec<i32>, _>>()
            })
            .collect::<Result<_, _>>()?;

        match contents.len() {
            2 => Ok(Pallet {
                    side_length: contents[0][0],
                    priorities: contents[1].clone(),
                    cost: None,
                    weights: None,
                    capacities: None,
                }),
            4 => Ok(Pallet {
                    side_length: contents[0][0],
                    priorities: contents[1].clone(),
                    cost: match contents[0].len() {
                        3 => Some(contents[0][2]),
                        _ => None,
                    },
                    weights: Some(contents[2].clone()),
                    capacities: Some(contents[3].clone()),
                }),
            _ => Err("Invalid input file body".into()),
        }
    }
}

