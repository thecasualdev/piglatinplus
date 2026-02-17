mod config;
mod core;

pub struct Args {
    pub verbose: bool,
    pub output: Option<String>,
    pub input: Option<String>,
}

fn main() {

    let args = core::parser::parse().unwrap();

    // TODO: Figure out command line parsing
    // Build a custom argument parser :D

    println!("{:#?} {:#?} {:#?}", 
    args.input.unwrap_or(String::from("No input")), 
    args.output.unwrap_or(String::from("No input")), 
    args.verbose);

    println!("Hello, world!");
}
