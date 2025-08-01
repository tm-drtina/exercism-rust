#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            years: (s as f64) / 60.0 / 60.0 / 24.0 / 365.25,
        }
    }
}

pub trait Planet {
    const SCALE: f64;

    fn years_during(d: &Duration) -> f64 {
        d.years / Self::SCALE
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
    const SCALE: f64 = 0.2408467;
}
impl Planet for Venus {
    const SCALE: f64 = 0.61519726;
}
impl Planet for Earth {
    const SCALE: f64 = 1.0;
}
impl Planet for Mars {
    const SCALE: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const SCALE: f64 = 11.862615;
}
impl Planet for Saturn {
    const SCALE: f64 = 29.447498;
}
impl Planet for Uranus {
    const SCALE: f64 = 84.016846;
}
impl Planet for Neptune {
    const SCALE: f64 = 164.79132;
}
