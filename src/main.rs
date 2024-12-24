mod binary_operations;
mod natures;
mod pokemon;
mod rng_factory;
use pokemon::Pokemon;
use rng_factory::RngFactory;

fn main() {
    // hardcoded input
    let gender_threshold: u8 = 126;
    let trainer_id: u16 = 54321; // Example Trainer ID
    let trainer_secret_id: u16 = 12345; // Example Secret ID

    let mut rng_factory = RngFactory::new();
    let personality_value = rng_factory.generate_personality_value();
    println!("Random u32: {}", personality_value);

    let last_byte = (personality_value & 0xFF) as u8;
    println!("Last byte: {}", last_byte);

    let some_pokemon = Pokemon::new(
        personality_value,
        gender_threshold,
        trainer_id,
        trainer_secret_id,
    );

    println!("{:#?}", some_pokemon);
}
