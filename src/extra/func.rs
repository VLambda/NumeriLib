pub(crate) struct Extra;

impl Extra {
    pub(crate) fn round(a: f64) -> f64 {
        let precision = 1e-6;

        let a1 = a * 1e+6;

        if a1.fract().abs() <= precision || a.fract().abs() <= precision {
            a1.floor() / 1e+6
        } else if a1.fract().abs() >= 1_f64 - precision || a.fract().abs() <= 1_f64 - precision {
            a1.ceil() / 1e+6
        } else {
            a
        }
    }

    pub const EPSILON1: f64 = 1e-7;

    pub const EPSILON2: f64 = 1e-15;
}
