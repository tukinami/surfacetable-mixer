use clap::Parser;

mod ast;
mod config;
mod process;

fn main() {
    let config = config::Config::parse();
    config.run();
}
