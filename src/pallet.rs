use std::fs;
use std::error::Error;

pub struct PalletConfig {
    pub l: i32,
    pub p: Vec<i32>,
    pub model: PalletModel,
}

pub enum PalletModel {
    Model1,
    Model2 {
        h: i32,
        w: Vec<i32>,
        c: Vec<i32>,
    },
    Model3 {
        h: i32,
        w: Vec<i32>,
        c: Vec<i32>,
    },
    Model4 {
        h: i32,
        k: i32,
        w: Vec<i32>,
        c: Vec<i32>,
    },
}

impl PalletConfig {
    pub fn from_input(path: &str, model: u8) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let mut rows = text.lines().map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });
        
        let header = rows.next().unwrap();
        
        Ok(PalletConfig {
            l: header[0],
            p: rows.next().unwrap(),
            model: match model {
                1 => PalletModel::Model1,
                2 => PalletModel::Model2 {
                    h: header[1],
                    w: rows.next().unwrap(),
                    c: rows.next().unwrap(),
                },
                3 => PalletModel::Model3 {
                    h: header[1],
                    w: rows.next().unwrap(),
                    c: rows.next().unwrap(),
                },
                4 => PalletModel::Model4 {
                    h: header[1],
                    k: header[2],
                    w: rows.next().unwrap(),
                    c: rows.next().unwrap(),
                },
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "could not match input model"))?,
            },
        })
    }
}
