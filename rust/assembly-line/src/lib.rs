pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total = (speed as f64) * 221.0;
    match speed {
        0 => 0.0,
        1..=4 => total,
        5..=8 => 0.9 * total,
        9..=10 => 0.77 * total,
        _ => panic!("assembly line production speed *must* be between 0 and 10 (inclusive)"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) as u32) / 60
}
