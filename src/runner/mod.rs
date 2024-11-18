// src/runner/mod.rs
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Runner {
    runner_dir: PathBuf,
}
impl Default for Runner {
    fn default() -> Self {
        Self::new("runner")
    }
}

impl Runner {
    pub fn new<T>(path: T) -> Self
    where
        T: AsRef<Path>,
    {
        Self {
            runner_dir: path.as_ref().to_path_buf(),
        }
    }

    pub fn check_day(&self, day: u8) -> String {
        let output = Command::new("cargo")
            .arg("check")
            .arg("-p")
            .arg(format!("day{:02}", day))
            .output();
        match output {
            Err(e) => e.to_string(),
            Ok(output) => String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }

    pub fn test_day(&self, day: u8) -> String {
        let output = Command::new("cargo")
            .arg("test")
            .arg("-p")
            .arg(format!("day{:02}", day))
            .output();
        match output {
            Err(e) => e.to_string(),
            Ok(output) => String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }

    pub fn run_day(&self, day: u8, part: u8, input: &str) -> Result<usize> {
        // Create temporary runner project
        let day_str = format!("day{:02}", day);
        // Write input file
        let input_path = self.runner_dir.join("input.txt");
        fs::write(&input_path, input)?;

        // Create Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "aoc-runner"
version = "0.1.0"
edition = "2021"

[dependencies]
{} = {{ path = "../{}" }}
"#,
            day_str, day_str
        );
        fs::write(self.runner_dir.join("Cargo.toml"), cargo_toml)?;

        // Create src directory
        fs::create_dir_all(self.runner_dir.join("src"))?;

        // Create main.rs
        let main_rs = format!(
            r#"fn main() {{
    println!("{{}}", {}::part{}(include_str!("../input.txt")));
}}"#,
            day_str, part
        );
        fs::write(self.runner_dir.join("src").join("main.rs"), main_rs)?;

        // Build and run
        Command::new("cargo")
            .arg("build")
            .current_dir(&self.runner_dir)
            .output()
            .context("Failed to build runner")?;

        let output = Command::new("cargo")
            .arg("run")
            .current_dir(&self.runner_dir)
            .output()
            .context("Failed to run solution")?;

        let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        result_str
            .parse()
            .context("Failed to parse result as usize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};
    use tempfile::TempDir;

    fn setup_test_day(workspace: &Path, day: u8) -> Result<()> {
        let day_path = workspace.join(format!("day{:02}", day));
        fs::create_dir_all(day_path.join("src"))?;

        // Create Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "day{:02}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
            day
        );
        fs::write(day_path.join("Cargo.toml"), cargo_toml)?;

        // Create lib.rs with a simple implementation
        let lib_rs = r#"
pub fn part1(input: &str) -> usize {
    42
}

pub fn part2(input: &str) -> usize {
    84
}
"#;
        fs::write(day_path.join("src").join("lib.rs"), lib_rs)?;

        Ok(())
    }

    #[test]
    fn test_runner_creation() -> Result<()> {
        let tempdir = TempDir::new()?;
        let _runner = Runner::new(tempdir);
        Ok(())
    }

    #[test]
    fn test_run_day() -> Result<()> {
        // Create a temporary workspace
        let workspace = TempDir::new()?;
        setup_test_day(workspace.path(), 1)?;

        // Create workspace Cargo.toml
        let workspace_toml = r#"[workspace]
members = ["day*"]
resolver = "2"
"#;
        fs::write(workspace.path().join("Cargo.toml"), workspace_toml)?;

        // Initialize runner
        let tempdir = TempDir::new_in(workspace.path())?;
        let runner = Runner::new(tempdir);

        // Test part 1
        let result = runner.run_day(1, 1, "test input")?;
        assert_eq!(result, 42);

        // Test part 2
        let result = runner.run_day(1, 2, "test input")?;
        assert_eq!(result, 84);

        Ok(())
    }

    #[test]
    fn test_check_and_test_day() -> Result<()> {
        // Create a temporary workspace
        let workspace = TempDir::new()?;
        setup_test_day(workspace.path(), 1)?;

        // Create workspace Cargo.toml
        let workspace_toml = r#"[workspace]
members = ["day*"]
resolver = "2"
"#;
        fs::write(workspace.path().join("Cargo.toml"), workspace_toml)?;

        // Initialize runner
        let tempdir = TempDir::new_in(workspace.path())?;
        let runner = Runner::new(tempdir);

        // Test cargo check
        let check_output = runner.check_day(1);
        assert!(!check_output.contains("error"));

        // Test cargo test
        let test_output = runner.test_day(1);
        assert!(!test_output.contains("failed"));

        Ok(())
    }
}
