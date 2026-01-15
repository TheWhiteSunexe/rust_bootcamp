// Exercice 01
// Author : Tristan Gillet

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut name = "World".to_string();
    let mut caps = false;
    let mut repeat = 1;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];

        if arg == "-h" || arg == "--help" {
            println!("Usage: rusty_hello [OPTIONS] [NAME]");
            println!("--upper       mettre en majuscules");
            println!("--repeat <N>  répéter N fois (par défaut 1)");
            println!("-h, --help    afficher ce message");
            return;
        } else if arg == "--upper" {
            caps = true;
        } else if arg == "--repeat" {
            i += 1;
            if i >= args.len() {
                println!("Oups, pas de valeur pour --repeat");
                return;
            }
            repeat = args[i].parse::<usize>().unwrap_or(1);
        } else if arg.starts_with("--") {
            println!("Option inconnue: {}", arg);
            return;
        } else {
            // argument positionnel pour le nom
            name = arg.to_string();
        }

        i += 1;
    }

    let mut message = format!("Hello, {}!", name);
    if caps {
        message = message.to_uppercase();
    }

    for _ in 0..repeat {
        println!("{}", message);
    }
}
