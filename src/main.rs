mod natures;
mod pokemon;
mod rng_factory;
use std::time::Instant;

use pokemon::Pokemon;
use rng_factory::RngFactory;

use polars::prelude::*;
use std::fs::File;
use std::io::Result;
use std::io::{self, Write};

const N_POKEMON: u32 = 1_000;
const GENDER_THRESHOLD: u8 = 126;
const TRAINER_ID: u16 = 54321; // Example Trainer ID
const TRAINER_SECRET_ID: u16 = 12345; // Example Secret ID
const OUTPUT_FILE_PATH: &str = "output/output.csv";

fn main() -> Result<()> {
    let start = Instant::now();

    let mut file = File::create(OUTPUT_FILE_PATH)?;
    file.write_all(b"personality_value,gender,nature,shiny_value,shiny,hp_iv,atk_iv,def_iv,spatk_iv,spdef_iv,speed_iv\n")?;

    let mut rng_factory = RngFactory::new();

    for _ in 0..N_POKEMON {
        let personality_value = rng_factory.generate_personality_value();
        let some_pokemon = Pokemon::new(
            personality_value,
            GENDER_THRESHOLD,
            TRAINER_ID,
            TRAINER_SECRET_ID,
        );
        file.write_all(some_pokemon.to_string().as_bytes())?;
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}
