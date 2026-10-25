use std::io;
use std::cmp::Ordering;

use rand::prelude::*;

fn main() {
    println!("Tebak angka rahasia");

    let secret_number = rand::rng()
        .random_range(1..=100);

    println!("Silakan masukkan tebakan Anda.");
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Gagal mambaca baris");

        let guess: u32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Tolong masukkan angka!");
                    continue;
                }
            };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Terlalu kecil"),
            Ordering::Greater => println!("Telalu besar"),
            Ordering::Equal => {
                println!("Anda menang!");
                break;
            }
        }
    }

    println!("Angkanya adalah: {secret_number}")
}
