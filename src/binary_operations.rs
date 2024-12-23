pub fn get_last_byte(value: &u32) -> u8 {
    (value & 0xFF) as u8
}
