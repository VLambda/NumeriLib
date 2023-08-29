use crate::special::Gamma;

pub struct Chi;

impl Chi {
    pub fn pdf(x: f64, df: f64) -> f64 {
        if x >= 0_f64 {
            let p1 = x.powf(df - 1_f64) * std::f64::consts::E.powf(-(x.powi(2) / 2_f64));
            let p2 = 2_f64.powf((df / 2_f64) - 1_f64) * Gamma::lanczos(df / 2_f64);
            p1 / p2
        } else {
            0_f64
        }
    }

    pub fn cdf(x: f64, df: f64) -> f64 {
        Gamma::reggamma(df / 2_f64, x.powi(2) / 2_f64)
    }

    pub fn mean(df: f64) -> f64 {
        std::f64::consts::SQRT_2
            * (Gamma::lanczos((df + 1_f64) / 2_f64) / Gamma::lanczos(df / 2_f64))
    }

    pub fn median(df: f64) -> f64 {
        (df * (1_f64 - (2_f64 / (9_f64 * df))).powi(3)).sqrt()
    }

    pub fn mode(df: f64) -> f64 {
        if df < 1_f64 {
            return 0_f64;
        }
        (df - 1_f64).sqrt()
    }

    pub fn variance(df: f64) -> f64 {
        df - Self::mean(df).powi(2)
    }

    pub fn sd(df: f64) -> f64 {
        Self::variance(df).sqrt()
    }

    pub fn skewness(df: f64) -> f64 {
        let mu = Self::mean(df);
        let omega = Self::sd(df);
        (mu / omega.powi(3)) * (1_f64 - 2_f64 * omega.powi(2))
    }

    pub fn kurtosis(df: f64) -> f64 {
        let sd = Self::sd(df);
        let skewness = Self::skewness(df);
        let mu = Self::mean(df);
        (2_f64 / sd.powi(2)) * (1_f64 - mu * sd * skewness - sd.powi(2))
    }
}
