use clap::Parser;

#[derive(Parser)]
#[command(name="forecast")]
#[command(about="Weather in your terminal", long_about=None)]
struct Args {
    /// Number of days for the forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.days);
}
