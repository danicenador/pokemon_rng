use crate::binary_operations;
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
}

impl Pokemon {
    pub fn new(personality_value: u32, gender_threshold: u8) -> Self {
        let gender = Pokemon::assign_gender(&personality_value, gender_threshold);
        let nature = natures::determine_nature(&personality_value);
        Pokemon {
            personality_value,
            gender,
            nature,
        }
    }
    fn assign_gender(pvalue: &u32, gender_threshold: u8) -> Gender {
        if gender_threshold == 255 {
            return Gender::Unknown;
        }

        let last_byte = binary_operations::get_last_byte(pvalue);
        if last_byte >= gender_threshold {
            Gender::Male
        } else {
            Gender::Female
        }
    }
}
