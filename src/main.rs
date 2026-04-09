mod app;
mod cli;
mod commands;
mod config;
mod domain;
mod generator;
pub mod model;

fn main() {
    cli::run();
}
