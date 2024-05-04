use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[arg(short='f', long="final", help="Print only the number requested")]
    final_number_only: bool,
    #[arg(short='a', long="all", help="Print all numbers up to number requested")]
    all_numbers: bool,
    #[arg(required=true, value_name="fibonacci number")]
    fibonacci_number: u32,
}
fn main() {
    let cli =Cli::parse();

    if cli.final_number_only {
        println!("Printing final fibonacci number");
    }

    if cli.all_numbers {
        println!("Printing all fibonacci numbers");
    }

    println!("number: {:?}", cli.fibonacci_number);
}
