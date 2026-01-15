use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process;

// Affiche l'aide du programme
fn print_help() {
    println!(
        "Usage: hex_tool [OPTIONS]

Read and write binary files in hexadecimal

Options:
-f, --file <FILE>        Target file
-r, --read               Read mode (display hex)
-w, --write <HEX>        Write mode (hex string to write)
-o, --offset <OFF>       Offset in bytes (decimal or 0x hex)
-s, --size <N>           Number of bytes to read
-h, --help               Print help"
    );
}

// Convertit une chaîne en offset, supporte décimal ou hex (0x..)
fn parse_offset(s: &str) -> u64 {
    if let Some(hex) = s.strip_prefix("0x") {
        // pour hex
        u64::from_str_radix(hex, 16).unwrap_or_else(|_| {
            eprintln!("error");
            process::exit(2);
        })
    } else {
        // pour décimal
        s.parse::<u64>().unwrap_or_else(|_| {
            eprintln!("error");
            process::exit(2);
        })
    }
}

// Convertir une chaîne hex en bytes
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    if hex.len() % 2 != 0 {
        eprintln!("error");
        process::exit(2);
    }

    let mut out = Vec::new();
    let chars: Vec<char> = hex.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let byte = u8::from_str_radix(
            &format!("{}{}", chars[i], chars[i + 1]),
            16,
        )
            .unwrap_or_else(|_| {
                eprintln!("error");
                process::exit(2);
            });
        out.push(byte);
        i += 2;
    }

    out
}

// Vérifie si tout les bytes sont des caractère affichables
fn is_printable(buf: &[u8]) -> bool {
    buf.iter().all(|b| b.is_ascii_graphic() || *b == b' ')
}

// Affichage type hexdump pour les données qui ne sont pas du texte
fn hexdump(offset: u64, buf: &[u8]) {
    // Affichage
    print!("{:08x}: ", offset);

    // Affiche les bytes en hex
    for i in 0..16 {
        if i < buf.len() {
            print!("{:02x} ", buf[i]);
        } else {
            print!(".. ");
        }
    }

    // Affichage de la partie lisible (ou un "." pour du non-ASCII)
    print!(" |");
    for b in buf {
        if b.is_ascii_graphic() || *b == b' ' {
            print!("{}", *b as char);
        } else {
            print!(".");
        }
    }
    println!("|");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Variables
    let mut file = None;
    let mut read_mode = false;
    let mut write_hex = None;
    let mut offset = 0u64;
    let mut size = None;

    // Récupération de l'argument
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-f" | "--file" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error");
                    process::exit(2);
                }
                file = Some(args[i].clone());
            }
            "-r" | "--read" => {
                read_mode = true;
            }
            "-w" | "--write" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error");
                    process::exit(2);
                }
                write_hex = Some(args[i].clone());
            }
            "-o" | "--offset" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error");
                    process::exit(2);
                }
                offset = parse_offset(&args[i]);
            }
            "-s" | "--size" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error");
                    process::exit(2);
                }
                size = Some(args[i].parse::<usize>().unwrap_or_else(|_| {
                    eprintln!("error");
                    process::exit(2);
                }));
            }
            _ => {
                // Option inconnue -> erreur
                eprintln!("error");
                process::exit(2);
            }
        }
        i += 1;
    }

    // Vérifie qu'un fichier est bien spécifié
    let file_path = file.unwrap_or_else(|| {
        eprintln!("error");
        process::exit(2);
    });

    if let Some(hex) = write_hex {
        // Convertit la chaîne hex en bytes
        let bytes = hex_to_bytes(&hex);

        // Ouvre le fichier (création si inexistant)
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&file_path)
            .unwrap_or_else(|_| {
                eprintln!("error");
                process::exit(2);
            });

        // Se positionne à l'offset souhaité
        f.seek(SeekFrom::Start(offset)).unwrap();

        // Écrit les bytes dans le fichier
        f.write_all(&bytes).unwrap();

        println!("Successfully written");
        return;
    }

    if read_mode {
        // Ouvre le fichier pour lecture
        let mut f = File::open(&file_path).unwrap_or_else(|_| {
            eprintln!("error");
            process::exit(2);
        });

        // Se positionne à l'offset souhaité
        f.seek(SeekFrom::Start(offset)).unwrap();

        // Lit le nombre de bytes demandé (ou 16 par défaut)
        let mut buf = vec![0u8; size.unwrap_or(16)];
        let read = f.read(&mut buf).unwrap();
        buf.truncate(read); // ajuste la taille réelle lue

        // Si le contenu est lisible, affiche en texte
        if is_printable(&buf) {
            println!("{}", String::from_utf8_lossy(&buf));
        } else {
            // Sinon affiche en hex
            hexdump(offset, &buf);
        }

        return;
    }

    // Aucun mode sélectionné -> erreur
    eprintln!("error : aucun mode séléctionné");
    process::exit(2);
}
