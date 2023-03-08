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
    const ORBITAL_YEARS: f64;
    const EARTH_ORBITAL_SECS: f64 = 31557600.0;
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (Self::ORBITAL_YEARS * Self::EARTH_ORBITAL_SECS)
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

impl Planet for Mercury {
    const ORBITAL_YEARS: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBITAL_YEARS: f64 = 0.6151972;
}
impl Planet for Earth {
    const ORBITAL_YEARS: f64 = 1.0000000;
}
impl Planet for Mars {
    const ORBITAL_YEARS: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_YEARS: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_YEARS: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_YEARS: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_YEARS: f64 = 164.79132;
}
