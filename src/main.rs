use std::char::from_u32;

use rand::Rng;

fn main() {
    
    let password_longitud: u32 = 80;

    let mut generador: String = String::new();

    for _ in 0..password_longitud {
    let número:u32 = rand::thread_rng().gen_range(0..2048);
    let char: char = from_u32(número).unwrap();
    generador.push(char);
    }
    let generador: &str = generador.trim();

    println!("The Password is:`\n{}\n`", generador)
}
