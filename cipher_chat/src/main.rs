use std::io::{self, BufRead, Write}; // Entrée/sortie standard, lecture tamponnée
use std::net::{TcpListener, TcpStream}; // Connexions réseau TCP
use std::thread; // Multithreading
use rand::Rng; // Génération de nombres aléatoires
use std::str::FromStr; // Conversion de chaînes en types
use std::sync::{Arc, Mutex}; // Partage sécurisé entre threads
use std::env; // Accès aux variables d'environnement

// Paramètres Diffie-Hellman 
const P: u64 = 0xD87FA3E291B4C7F3; // Modulo premier
const G: u64 = 2; // Base/générateur

// Exponentiation modulaire rapide
// Calcule (base^exp) % modulo efficacement en utilisant l'exponentiation binaire
fn mod_exp(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1;
    base %= modulo; // Réduction initiale pour éviter overflow
    while exp > 0 {
        if exp % 2 == 1 { // Si le bit courant de l'exposant est 1
            result = (result * base) % modulo;
        }
        base = (base * base) % modulo; // Carré de la base
        exp /= 2; // Décalage de l'exposant
    }
    result
}


// XOR stream cipher
fn xor_cipher(data: &[u8], key: u64) -> Vec<u8> {
    let mut keystream = key;
    data.iter().map(|b| {
        let k = (keystream & 0xFF) as u8;
        keystream = keystream.rotate_left(8);
        b ^ k
    }).collect()
}

// Diffie-Hellman key generation
fn dh_generate_keypair() -> (u64, u64) {
    let private = rand::thread_rng().gen::<u64>();
    let public = mod_exp(G, private, P);
    (private, public)
}

fn dh_compute_secret(their_public: u64, private: u64) -> u64 {
    mod_exp(their_public, private, P)
}

fn handle_stream(mut stream: TcpStream, secret: u64) {
    let secret = Arc::new(Mutex::new(secret));

    // Thread pour lire le flux entrant
    let read_secret = Arc::clone(&secret);
    let mut stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || {
        let mut buf = [0u8; 512];
        loop {
            match stream_clone.read(&mut buf) {
                Ok(0) => break, // déconnexion
                Ok(n) => {
                    let decrypted = xor_cipher(&buf[..n], *read_secret.lock().unwrap());
                    print!("> {}", String::from_utf8_lossy(&decrypted));
                }
                Err(_) => break,
            }
        }
    });

    // Écrire sur le flux
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap() + "\n";
        let encrypted = xor_cipher(line.as_bytes(), *secret.lock().unwrap());
        stream.write_all(&encrypted).unwrap();
    }
}

fn server(port: &str) {
    // Crée un serveur TCP écoutant sur toutes les interfaces au port donné
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("[SERVER] Listening on 0.0.0.0:{}", port);

    // Attend une connexion entrante
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("[CLIENT] Connected from {}", stream.peer_addr().unwrap());

        // Échange de clés Diffie-Hellman côté serveur
        println!("[DH] Starting key exchange...");
        let (private, public) = dh_generate_keypair();

        // Envoie la clé publique du serveur
        stream.write_all(&public.to_be_bytes()).unwrap();

        // Reçoit la clé publique du client
        let mut their_pub_bytes = [0u8; 8];
        stream.read_exact(&mut their_pub_bytes).unwrap();
        let their_public = u64::from_be_bytes(their_pub_bytes);

        // Calcule le secret partagé
        let secret = dh_compute_secret(their_public, private);
        println!("[DH] Shared secret: {:X}", secret);

        // Lance la communication chiffrée avec le client
        handle_stream(stream, secret);
        break; // Un seul client géré
    }
}

fn client(addr: &str) {
    // Connexion TCP au serveur
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("[CLIENT] Connected to {}", addr);

    // Échange de clés Diffie-Hellman côté client
    println!("[DH] Starting key exchange...");
    let (private, public) = dh_generate_keypair();

    // Reçoit la clé publique du serveur
    let mut their_pub_bytes = [0u8; 8];
    stream.read_exact(&mut their_pub_bytes).unwrap();
    let their_public = u64::from_be_bytes(their_pub_bytes);

    // Envoie la clé publique du client
    stream.write_all(&public.to_be_bytes()).unwrap();

    // Calcule le secret partagé
    let secret = dh_compute_secret(their_public, private);
    println!("[DH] Shared secret: {:X}", secret);

    // Lance la communication chiffrée avec le serveur
    handle_stream(stream, secret);
}

fn main() {
    // Récupère les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <server PORT> | <client IP:PORT>", args[0]);
        return;
    }

    // Sélection du mode serveur ou client selon les arguments
    match args[1].as_str() {
        "server" if args.len() == 3 => server(&args[2]),
        "client" if args.len() == 3 => client(&args[2]),
        _ => println!("Invalid arguments"),
    }
}
