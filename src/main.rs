mod cli;
mod rsa;

use cli::{Cli, Commands};
use clap::Parser;
use rsa::keygen::generate_rsa_keypair;
use rsa::encryption::{encrypt, decrypt};
use rsa::signature::{sign, verify};
use std::fs::File;
use std::io::{self, Read, Write};
use rug::Integer;


fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Keygen { pubkey, privkey, bits } => {
            let (e, d, n) = generate_rsa_keypair(bits);

            let mut pub_file = File::create(pubkey).expect("Failed to create public key file");
            writeln!(pub_file, "{}\n{}", e, n).expect("Failed to write public key");

            let mut priv_file = File::create(privkey).expect("Failed to create private key file");
            writeln!(priv_file, "{}\n{}", d, n).expect("Failed to write private key");
        }
        Commands::Encrypt { pubkey, r#in, out } => {
            let mut key_data = String::new();
            File::open(pubkey).expect("Failed to open public key").read_to_string(&mut key_data).unwrap();
            let mut lines = key_data.lines();
            let e = lines.next().unwrap().parse::<Integer>().unwrap();
            let n = lines.next().unwrap().parse::<Integer>().unwrap();

            let mut buf = Vec::new();
            match r#in {
                Some(input) => File::open(input).unwrap().read_to_end(&mut buf).unwrap(),
                None => std::io::stdin().read_to_end(&mut buf).unwrap(),
            };

            let plain = String::from_utf8(buf).expect("Failed to read input as string");
            let cipher = encrypt(&plain, &e, &n);

            let mut output = match out {
                Some(output_path) => Box::new(File::create(output_path).unwrap()) as Box<dyn Write>,
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };
            writeln!(output, "{:X}", cipher).unwrap();
        }
        Commands::Decrypt { privkey, r#in, out } => {
            let mut key_data = String::new();
            File::open(privkey).expect("Failed to open private key").read_to_string(&mut key_data).unwrap();
            let mut lines = key_data.lines();
            let d = lines.next().unwrap().parse::<Integer>().unwrap();
            let n = lines.next().unwrap().parse::<Integer>().unwrap();

            let mut s = String::new();
            match r#in {
                Some(input) => File::open(input).unwrap().read_to_string(&mut s).unwrap(),
                None => std::io::stdin().read_to_string(&mut s).unwrap(),
            };

            let c = Integer::from_str_radix(s.trim(), 16).expect("Invalid ciphertext format");
            let plain = decrypt(&c, &d, &n);

            let mut output = match out {
                Some(output_path) => Box::new(File::create(output_path).unwrap()) as Box<dyn Write>,
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };
            output.write_all(&plain).unwrap();
        }
        Commands::Sign { privkey, r#in, out } => {
            let mut key_data = String::new();
            File::open(privkey).expect("Failed to open private key").read_to_string(&mut key_data).unwrap();
            let mut lines = key_data.lines();
            let d = lines.next().unwrap().parse::<Integer>().unwrap();
            let n = lines.next().unwrap().parse::<Integer>().unwrap();

            let mut buf = Vec::new();
            match r#in {
                Some(input) => File::open(input).unwrap().read_to_end(&mut buf).unwrap(),
                None => std::io::stdin().read_to_end(&mut buf).unwrap(),
            };

            let sig = sign(&buf, &d, &n);

            let mut output = match out {
                Some(output_path) => Box::new(File::create(output_path).unwrap()) as Box<dyn Write>,
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };
            writeln!(output, "{:X}", sig).unwrap();
        }
        Commands::Verify { pubkey, r#in, sig } => {
            let mut key_data = String::new();
            File::open(pubkey).expect("Failed to open public key").read_to_string(&mut key_data).unwrap();
            let mut lines = key_data.lines();
            let e = lines.next().unwrap().parse::<Integer>().unwrap();
            let n = lines.next().unwrap().parse::<Integer>().unwrap();

            let mut buf = Vec::new();
            match r#in {
                Some(input) => File::open(input).unwrap().read_to_end(&mut buf).unwrap(),
                None => std::io::stdin().read_to_end(&mut buf).unwrap(),
            };

            let mut sig_str = String::new();
            File::open(sig).unwrap().read_to_string(&mut sig_str).unwrap();
            let sig = Integer::from_str_radix(sig_str.trim(), 16).expect("Invalid signature format");

            if verify(&buf, &sig, &e, &n) {
                println!("Signature is valid");
            } else {
                println!("Signature is invalid");
            }
        }
    }
}

