use clap::Parser;

/// simple cli tool for greeting a person
#[derive(Parser)]
#[command(name = "greet")]
#[command(about = "A simple CLI greeting tool")]

struct Cli{
    /// The name of the person to be greeted
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
