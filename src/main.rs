use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[arg(short, long)]
    option: Option<String>,
    fibonacci_number: u32,
}
fn main() {
    let cli =Cli::parse();

    println!("option, number: {:?}, {:?}",cli.option.as_deref(),cli.fibonacci_number);
}
