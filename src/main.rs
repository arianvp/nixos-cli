use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {}
fn main() {
    let _cli = Cli::parse();
}
