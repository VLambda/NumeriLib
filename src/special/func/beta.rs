use crate::special::Gamma;
use crate::special::Functions;

pub struct Beta;

impl Beta {
    pub fn lnbeta(z1: f64, z2: f64) -> f64 {
        (Gamma::lanczosln(z1) * Gamma::lanczosln(z2)) / Gamma::lanczosln(z1 + z2)
    }

    pub fn beta(z1:f64, z2:f64) -> f64 {
        Beta::lnbeta(z1, z2).exp()
    }

    pub fn incbeta(x: f64, z1: f64, z2: f64) -> f64 {
        let upper = x;
        let lower = 0_f64;
        let e1 = z1 - 1_f64;
        let e2 = z2 - 1_f64;
        let function = |t: f64| t.powf(e1) * (1_f64 - t).powf(e2);
        let eval = Functions::integral(lower, upper, function).unwrap();
        eval
    }

    pub fn regincbeta(x: f64, z1: f64, z2: f64) -> f64 {
        Beta::incbeta(x, z1, z2) / Beta::beta(z1, z2)
    }

}