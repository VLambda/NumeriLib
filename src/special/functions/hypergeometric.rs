use crate::special::Gamma;
use crate::Functions;

pub struct Hypergeometric;

impl Hypergeometric {
    pub fn gaussian(a: f64, b: f64, c: f64, z: f64) -> f64 {
        let mut result = 0_f64;

        for k in 0..70 {
            result += ((Functions::pochhammer(a, k as f64) * Functions::pochhammer(b, k as f64))
                / Functions::pochhammer(c, k as f64))
                * (z.powf(k as f64) / Functions::factorial(k as f64))
        }

        result
    }

    pub fn kummer(a: f64, b: f64, z: f64) -> f64 {
        let mut result = 0_f64;

        for k in 0..99 {
            result += ((Functions::pochhammer(a, k as f64)) / Functions::pochhammer(b, k as f64))
                * (z.powf(k as f64) / Functions::factorial(k as f64))
        }

        result
    }

    pub fn tricomi(a: f64, b: f64, z: f64) -> f64 {
        (Gamma::lanczos(1_f64 - b) / Gamma::lanczos(a - b + 1_f64)) * Self::kummer(a, b, z)
            + (Gamma::lanczos(b - 1_f64) / Gamma::lanczos(a))
                * Self::kummer(a - b + 1_f64, 2_f64 - b, z)
    }

    pub fn whittaker(k: f64, m: f64, z: f64) -> f64 {
        std::f64::consts::E.powf(-z / 2_f64)
            * z.powf(m + (1_f64 / 2_f64))
            * Self::kummer(m - k + (1_f64 / 2_f64), (2_f64 * m) + 1_f64, z)
    }

    pub fn wittaker2(k: f64, m: f64, z: f64) -> f64 {
        std::f64::consts::E.powf(-z / 2_f64)
            * z.powf(m + (1_f64 / 2_f64))
            * Self::tricomi(m - k + (1_f64 / 2_f64), (2_f64 * m) + 1_f64, z)
    }
}
