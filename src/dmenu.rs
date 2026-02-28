use anyhow::{Result, bail};

use crate::models::MenuItem;

// Reads all lines from stdin, and stops
// when EOF reached.
pub fn read_input_lines() -> Result<Vec<String>> {
    let mut lines = Vec::<String>::new();
    let mut buffer = String::new();
    while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        if size == 0 {
            return Ok(lines);
        }

        lines.push(buffer.clone());
        buffer.clear();
    }

    bail!("could not read from stdin")
}

// Converts the input test into an array
// of MenuItems. It assumes that every line
// is an item. The resulting MenuItem will simply
// have the command to print the selection.
pub fn items_from_str(lines: &[String]) -> Vec<MenuItem> {
    lines
        .iter()
        .map(|l| MenuItem {
            text: l.to_string(),
            name: l.to_string(),
            ..Default::default()
        })
        .collect()
}

