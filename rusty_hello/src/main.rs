use std::collections::HashMap;
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut top = 10;
    let mut min_len = 1;
    let mut ignore_case = false;
    let mut text_arg: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            println!("Usage: wordfreq [OPTIONS] <TEXT>");
            println!("--top <N> pour montrer le Top X des mots (par defaut 10)");
            println!(
                "--min-length <N> ignore les mots avec une certaine taille minimum (par defaut 1)"
            );
            println!("--ignore-case pour ignorer les majuscules");
            println!("-h, --help afficher l'aide");
            return;
        } else if arg == "--top" {
            i += 1;
            if i >= args.len() {
                println!("Oups, pas de valeur pour --top, on prend 10");
            } else {
                top = args[i].parse::<usize>().unwrap_or(10);
            }
        } else if arg == "--min-length" {
            i += 1;
            if i >= args.len() {
                println!("Oups, pas de valeur pour --min-length, on prend 1");
            } else {
                min_len = args[i].parse::<usize>().unwrap_or(1);
            }
        } else if arg == "--ignore-case" {
            ignore_case = true;
        } else {
            text_arg = Some(arg.to_string());
        }
        i += 1;
    }

    // lire stdin si aucun texte passé en argument
    let mut text = if let Some(t) = text_arg {
        t
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or(0);
        buf
    };

    if ignore_case {
        text = text.to_lowercase();
    }

    let mut freqs: HashMap<String, usize> = HashMap::new();

    for word in text.split(|c: char| !c.is_alphanumeric()) {
        if !word.is_empty() && word.len() >= min_len {
            *freqs.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut words: Vec<(&String, &usize)> = freqs.iter().collect();

    words.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    if top < words.len() {
        println!("Top {} words:", top);
    } else {
        println!("Word frequency:");
    }

    for (w, c) in words.into_iter().take(top) {
        println!("{w}: {c}");
    }
}
