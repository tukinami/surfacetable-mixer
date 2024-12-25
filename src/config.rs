use std::path::PathBuf;

use clap::Parser;

use crate::process::process;

const DEFAULT_TARGET_PATH: &str = "./surfaces.yaml";
const DEFAULT_OUTPUT_PATH: &str = "./surfacetable.txt";
const DEFAULT_SEPARATOR: &str = "-";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Config {
    /// Path to input file.
    #[arg(short, long, default_value = DEFAULT_TARGET_PATH)]
    input: PathBuf,
    /// Path to output file.
    #[arg(short, long, default_value = DEFAULT_OUTPUT_PATH)]
    output: PathBuf,
    /// Flag of force overwriting.
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Whitelist for surfaces, separated by comma.
    #[arg(short, long, default_value = None, value_parser = whitelist_in_csv)]
    whitelist: Option<std::vec::Vec<usize>>,
    /// Separator string for a parts of the surface.
    #[arg(short, long, default_value = DEFAULT_SEPARATOR)]
    separator: String,
}

fn whitelist_in_csv(s: &str) -> Result<Vec<usize>, String> {
    let mut whitelist = Vec::new();

    for (index, element) in s.split(',').enumerate() {
        match element.parse::<usize>() {
            Ok(v) => whitelist.push(v),
            Err(e) => {
                return Err(format!("Whitelist is invalid: element: {}: {}", index, e));
            }
        }
    }

    Ok(whitelist)
}

impl Config {
    #[cfg(test)]
    pub fn new(
        input: PathBuf,
        output: PathBuf,
        force: bool,
        whitelist: Option<Vec<usize>>,
        separator: String,
    ) -> Config {
        Config {
            input,
            output,
            force,
            whitelist,
            separator,
        }
    }

    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn force(&self) -> &bool {
        &self.force
    }

    pub fn whitelist(&self) -> Option<&Vec<usize>> {
        self.whitelist.as_ref()
    }

    pub fn separator(&self) -> &String {
        &self.separator
    }

    pub fn run(&self) {
        if let Err(err) = process(self) {
            eprintln!("Application error: {}", err);
            std::process::exit(1);
        }
    }
}
