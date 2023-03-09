const CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        0 => 0.00,
        1..=4 => 1.00,
        5..=8 => 0.90,
        9.. => 0.77,
    };
    speed as f64 * CARS_PER_HOUR * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
