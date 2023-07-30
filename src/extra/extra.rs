pub(crate) struct Extra;

impl Extra {
    pub(crate) fn round(a: f64) -> f64 {
        let precision = 1e-6; // Tolerance value

        let a1 = a * 1e+6;

        if (a1.floor() - a1).abs() <= precision {
            a1.floor() / 1e+6
        } else if (a1 - a1.ceil()).abs() <= precision {
            a1.ceil() / 1e+6
        } else {
            a
        }
    }
}