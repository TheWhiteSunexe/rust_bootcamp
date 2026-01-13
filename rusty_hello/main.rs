use std::env;
use std::process;

fn print_help() {
    println!(
        "\
Usage: rusty_hello [OPTIONS] [NAME]

Arguments:
  [NAME]  Name to greet [default: World]

Options:
  --upper       Convert to uppercase
  --repeat <N>  Repeat greeting N times [default: 1]
  -h, --help    Print help"
    );
}

fn parse_repeat(value: &str) -> usize {
    value.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: --repeat expects a positive integer");
        process::exit(1);
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut name = String::from("World");
    let mut upper = false;
    let mut repeat = 1;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--upper" => {
                upper = true;
            }
            "--repeat" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --repeat requires a value");
                    process::exit(1);
                }
                repeat = parse_repeat(&args[i]);
            }
            value => {
                // Argument positionnel : NAME (une seule fois)
                name = value.to_string();
            }
        }
        i += 1;
    }

    let greeting = format!("Hello, {}!", name);
    let output = if upper {
        greeting.to_uppercase()
    } else {
        greeting
    };

    for _ in 0..repeat {
        println!("{output}");
    }
}
