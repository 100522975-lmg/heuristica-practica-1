pub fn format_header(out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines = out.lines();
    match lines.take(6).collect::<Vec<&str>>().as_slice() {
        [_, rows, cols, _, _, cost] => Ok(println!(
            "{} {} {}",
            rows.split_whitespace().nth(1).unwrap_or(""),
            cols.split_whitespace().nth(1).unwrap_or(""),
            cost.split_whitespace().nth(3).unwrap_or(""),
        )),
        _ => Err("Could not read glpsol output header".into()),
    }
}
