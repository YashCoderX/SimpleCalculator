use clap::Parser;

/// Simple calculator
#[derive(Parser, Debug)]
#[command(name = "calc", about = "A simple calculator CLI tool")]
struct Args {
    /// First number
    #[arg(short = 'a', long = "num1")]
    num1: f64,

    /// Second number
    #[arg(short = 'b', long = "num2")]
    num2: f64,

    /// Operation to perform: add, sub, mul, div
    #[arg(short = 'o', long = "operation")]
    operation: String,
}

fn main() {
    let args = Args::parse();

    let result = match args.operation.as_str() {
        "add" => args.num1 + args.num2,
        "sub" => args.num1 - args.num2,
        "mul" => args.num1 * args.num2,
        "div" => {
            if args.num2 != 0.0 {
                args.num1 / args.num2
            } else {
                eprintln!("Error: Division by zero");
                return;
            }
        }
        _ => {
            eprintln!("Error: Unsupported operation. Use add, sub, mul, or div.");
            return;
        }
    };

    println!("Result: {}", result);
}
