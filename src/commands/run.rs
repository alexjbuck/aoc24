// src/commands/run.rs
use crate::runner::Runner;
use anyhow::Result;
use dialoguer::Editor;
use std::fs;
use std::path::PathBuf;

pub async fn execute(day: u8, part: u8, input_path: Option<PathBuf>) -> Result<()> {
    let runner = Runner::new()?;

    // Get input either from file or editor
    let input = if let Some(path) = input_path {
        fs::read_to_string(path)?
    } else {
        // Try to load from saved inputs first
        let input_file = format!("inputs/day{:02}/part{}.txt", day, part);
        if let Ok(content) = fs::read_to_string(&input_file) {
            content
        } else {
            // Open editor for input
            let input = Editor::new()
                .edit("# Enter your input here\n")?
                .ok_or_else(|| anyhow::anyhow!("Input was not provided"))?;

            // Save input for future use
            fs::create_dir_all(format!("inputs/day{:02}", day))?;
            fs::write(&input_file, &input)?;

            input
        }
    };

    // Run the solution
    let result = runner.run_day(day, part, &input)?;
    println!("Day {} Part {}: {}", day, part, result);

    Ok(())
}
