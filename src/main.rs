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
    fibonacci_number: f32,
}
fn main() {
    let cli = Cli::parse();
    let no_flag = cli.final_number_only == cli.all_numbers;

    if cli.final_number_only || no_flag {
        calculate_fibonacci_number(cli.fibonacci_number);
    }

    if cli.all_numbers {
        for i in 1..=cli.fibonacci_number as u32 {
            calculate_fibonacci_number(i as f32)
        }
    }
}

fn calculate_fibonacci_number(number: f32) {
    let sqrt_five = 5.0_f32.sqrt();
    let phi = (1.0_f32 + sqrt_five).powf(number);
    let psi = (1.0_f32 - sqrt_five).powf(number);

    let fibonacci = ((phi - psi) / (2_i32.pow(number as u32) as f32 * sqrt_five)) as u32;

    println!("{fibonacci}");
}
