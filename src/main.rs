use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'f', long = "final", help = "Print only the number requested")]
    final_number_only: bool,
    #[arg(
        short = 'a',
        long = "all",
        help = "Print all numbers up to number requested"
    )]
    all_numbers: bool,
    #[arg(required = true, value_name = "fibonacci number")]
    fibonacci_number: u32,
}
fn main() {
    let cli = Cli::parse();

    if cli.final_number_only {
        println!("Printing final fibonacci number");
    }

    if cli.all_numbers {
        for i in 1..=cli.fibonacci_number {
            println!("Printing all fibonacci numbers");
        }
    }

    println!("number: {:?}", cli.fibonacci_number);
}

fn calculate_fibonacci_number(number: u32) {
    let sqrt_five = f32::sqrt(5.0) as u32 + 1;
    let phi = (1 + sqrt_five).pow(number);
    let psi = (1 - sqrt_five).pow(number);

    let fibonacci = (phi - psi) / (2_u32.pow(number) * sqrt_five);

    println!("{fibonacci}");
}
