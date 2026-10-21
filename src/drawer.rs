use crate::pallet::{PalletConfig, PalletModel};
use std::error::Error;

pub struct Drawer {
    model: u8,
    length: usize,
    height: usize,
    boxes: usize,
    priorities: Vec<i32>,
    /// grid[pallet][level][row][col] -> priority of the box placed there (0 = empty).
    /// Dimensions that don't exist for a model collapse to size 1:
    ///   model 1 -> 1 pallet, 1 level; models 2/3 -> 1 pallet; model 4 -> full 4D.
    grid: Vec<Vec<Vec<Vec<i32>>>>,
}

/// Converts a 1-based GLPK index into a 0-based Rust index.
fn index(s: &str) -> Result<usize, Box<dyn Error>> {
    let v = s.parse::<usize>()?;
    Ok(v.checked_sub(1).ok_or("indices start at 1")?)
}

impl Drawer {
    /// `pallets` is only used for model 4 (pass an upper bound, e.g. the number
    /// of boxes; unused pallets are skipped when drawing).
    pub fn new(config: &PalletConfig, pallets: usize) -> Self {
        let (model, height) = match &config.model {
            PalletModel::Model1 => (1, 1),
            PalletModel::Model2 { h, .. } => (2, *h as usize),
            PalletModel::Model3 { h, .. } => (3, *h as usize),
            PalletModel::Model4 { h, .. } => (4, *h as usize),
        };
        let length = config.l as usize;
        let pallets = if model == 4 { pallets.max(1) } else { 1 };

        Drawer {
            model,
            length,
            height,
            boxes: config.p.len(),
            priorities: config.p.clone(),
            grid: vec![vec![vec![vec![0; length]; length]; height]; pallets],
        }
    }

    /// Parses the glpsol solution and fills the grid. Returns the number of placed boxes.
    pub fn parse(&mut self, out: &str) -> Result<usize, Box<dyn Error>> {
        let mut in_columns = false;
        let mut pending: Option<&str> = None; // name seen on the first line of a 2-line entry
        let mut placed = 0;

        for line in out.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens.as_slice() {
                // start of the columns table
                ["No.", "Column", ..] => {
                    in_columns = true;
                    pending = None;
                }
                _ if !in_columns => {}
                // name and value on the same line: "16 x[3,2,box1] * 1 0 1"
                [_, name, "*", val, ..] if name.starts_with("x[") => {
                    placed += self.place(name, val)?;
                }
                // long name on its own line: "16 x[3,2,2,box1]"
                [_, name] if name.starts_with("x[") => pending = Some(*name),
                // ...and its value on the next line: "* 1 0 1"
                ["*", val, ..] => {
                    if let Some(name) = pending.take() {
                        placed += self.place(name, val)?;
                    }
                }
                _ => {}
            }
        }

        if placed != self.boxes {
            return Err(format!("expected {} placed boxes, found {}", self.boxes, placed).into());
        }
        Ok(placed)
    }

    /// Writes the box priority into the grid if the variable equals 1.
    /// Returns 1 if a box was placed, 0 otherwise (value 0 or EmptyBox).
    fn place(&mut self, name: &str, val: &str) -> Result<usize, Box<dyn Error>> {
        // x is binary: glpsol prints exactly "0" or "1"
        if val != "1" {
            return Ok(0);
        }

        let inner = name
            .strip_prefix("x[")
            .and_then(|s| s.strip_suffix(']'))
            .ok_or("malformed variable name")?;
        let idx: Vec<&str> = inner.split(',').collect();

        // Right side follows the .mod declaration order; left side is grid order.
        let (p, h, i, j, b) = match (self.model, idx.as_slice()) {
            (1, &[i, j, b]) => ("1", "1", i, j, b),
            (2 | 3, &[i, j, h, b]) => ("1", h, i, j, b),
            (4, &[i, j, h, p, b]) => (p, h, i, j, b),
            _ => return Err(format!("unexpected variable `{name}` for model {}", self.model).into()),
        };

        // x[..., EmptyBox] means "no box here": leave the cell at 0
        let Some(n) = b.strip_prefix("box") else {
            return Ok(0);
        };
        let prio = *self
            .priorities
            .get(index(n)?)
            .ok_or_else(|| format!("unknown box `{b}`"))?;

        // convert indices first, so `?` is used on Results, not inside Option closures
        let (p, h, i, j) = (index(p)?, index(h)?, index(i)?, index(j)?);

        let cell = self
            .grid
            .get_mut(p)
            .and_then(|x| x.get_mut(h))
            .and_then(|x| x.get_mut(i))
            .and_then(|x| x.get_mut(j))
            .ok_or_else(|| format!("variable `{name}` is out of bounds"))?;

        *cell = prio;
        Ok(1)
    }
}
