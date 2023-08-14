pub struct Cauchy;

impl Cauchy {
    pub fn pdf(location: f64, scale: f64, x: f64) -> f64 {
        1_f64 / (std::f64::consts::PI * scale * ((x - location) / scale).powi(2))
    }

    pub fn cdf(location: f64, scale: f64, x: f64) -> f64 {
        std::f64::consts::FRAC_1_PI * ((x - location) / scale).atan() + (1_f64 / 2_f64)
    }

    pub fn inv(location: f64, scale: f64, probability: f64) -> f64 {
        location + scale * (std::f64::consts::PI * (probability - (1_f64 / 2_f64))).tan()
    }
}
