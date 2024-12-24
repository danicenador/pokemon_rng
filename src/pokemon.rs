use crate::natures;
use std::fmt::Debug;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug)]
pub struct Pokemon {
    personality_value: u32,
    gender: Gender,
    nature: natures::Nature,
    shiny_value: u16,
    shiny: bool,
    ivs: [u8; 6],
}

impl Pokemon {
    pub fn new(
        personality_value: u32,
        gender_threshold: u8,
        trainer_id: u16,
        trainer_secret_id: u16,
    ) -> Self {
        let gender = Pokemon::calculate_gender(&personality_value, gender_threshold);
        let nature = natures::determine_nature(&personality_value);
        let shiny_value =
            Pokemon::calculate_shiny_value(&personality_value, &trainer_id, &trainer_secret_id);
        let shiny = shiny_value < 8;
        let ivs = Pokemon::calculate_ivs(&personality_value);
        Pokemon {
            personality_value,
            gender,
            nature,
            shiny_value,
            shiny,
            ivs,
        }
    }
    fn calculate_gender(pvalue: &u32, gender_threshold: u8) -> Gender {
        if gender_threshold == 255 {
            return Gender::Unknown;
        }

        let last_byte = (pvalue & 0xFF) as u8;
        if last_byte >= gender_threshold {
            Gender::Male
        } else {
            Gender::Female
        }
    }
    fn calculate_shiny_value(pvalue: &u32, trainer_id: &u16, trainer_secret_id: &u16) -> u16 {
        let pid_high = (pvalue >> 16) as u16;
        let pid_low = (pvalue & 0xFFFF) as u16;
        pid_high ^ pid_low ^ trainer_id ^ trainer_secret_id
    }
    fn calculate_ivs(pvalue: &u32) -> [u8; 6] {
        let mut ivs = [0; 6];
        for i in 0..6 {
            ivs[i] = ((pvalue >> (i * 5)) & 0x1F) as u8;
        }
        ivs
    }
}
