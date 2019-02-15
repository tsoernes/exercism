#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const ORBITAL_SECS: f64;
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / Self::ORBITAL_SECS
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_ORBITAL_SECS: f64 = 31557600.0;

impl Planet for Mercury {
    const ORBITAL_SECS: f64 = 0.2408467 * EARTH_ORBITAL_SECS;
}
impl Planet for Venus {
    const ORBITAL_SECS: f64 = 0.6151972 * EARTH_ORBITAL_SECS;
}
impl Planet for Earth {
    const ORBITAL_SECS: f64 = 1.0000000 * EARTH_ORBITAL_SECS;
}
impl Planet for Mars {
    const ORBITAL_SECS: f64 = 1.8808158 * EARTH_ORBITAL_SECS;
}
impl Planet for Jupiter {
    const ORBITAL_SECS: f64 = 11.862615 * EARTH_ORBITAL_SECS;
}
impl Planet for Saturn {
    const ORBITAL_SECS: f64 = 29.447498 * EARTH_ORBITAL_SECS;
}
impl Planet for Uranus {
    const ORBITAL_SECS: f64 = 84.016846 * EARTH_ORBITAL_SECS;
}
impl Planet for Neptune {
    const ORBITAL_SECS: f64 = 164.79132 * EARTH_ORBITAL_SECS;
}
