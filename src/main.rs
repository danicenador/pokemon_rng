mod natures;
mod pokemon;
mod rng_factory;
use std::time::Instant;

use pokemon::Pokemon;
use rng_factory::RngFactory;

use polars::prelude::*;
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    // hardcoded input
    let number_of_pokemon: u32 = 1_000;
    let gender_threshold: u8 = 126;
    let trainer_id: u16 = 54321; // Example Trainer ID
    let trainer_secret_id: u16 = 12345; // Example Secret ID

    let start = Instant::now();

    let mut rng_factory = RngFactory::new();

    let first_pvalue = rng_factory.generate_personality_value();
    let first_pokemon = Pokemon::new(
        first_pvalue,
        gender_threshold,
        trainer_id,
        trainer_secret_id,
    );
    let mut result_df = first_pokemon.to_dataframe().unwrap();

    for _ in 0..number_of_pokemon - 1 {
        let personality_value = rng_factory.generate_personality_value();
        let some_pokemon = Pokemon::new(
            personality_value,
            gender_threshold,
            trainer_id,
            trainer_secret_id,
        );
        let pokemon_df = some_pokemon.to_dataframe();
        match pokemon_df {
            Ok(pokemon_df) => {
                result_df = result_df
                    .vstack(&pokemon_df)
                    .expect("Failed to stack DataFrames");
            }
            Err(e) => {
                eprintln!("Error converting Pokemon to DataFrame: {:?}", e);
            }
        };
    }
    // Write the DataFrame to a CSV file
    let file = File::create("output/output.csv")?;
    CsvWriter::new(&file).finish(&mut result_df).unwrap();
    println!("DataFrame written to output.csv");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}
