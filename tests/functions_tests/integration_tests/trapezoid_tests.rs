use numerilib::Functions;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn power_test() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let intervals = 6_f64;
        let function = |x: f64| x.powf(2_f64);

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(73_f64, integral);
    }

    #[test]
    pub fn polynomial_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(2.3337499999999998, integral);
    }

    #[test]
    pub fn trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sin();

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.4149665174494506, integral);
    }

    #[test]
    pub fn logarithmic_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.ln();

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.38619020963220585, integral);
    }

    #[test]
    pub fn exponential_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.exp();

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.7186397889252214, integral);
    }

    #[test]
    pub fn inverse_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| 1_f64 / x;

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.6933033817926941, integral);
    }

    #[test]
    pub fn nonelementary_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(x);

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.7846607642416727, integral);
    }

    #[test]
    pub fn hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sinh();

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.5431937719002027, integral);
    }

    #[test]
    pub fn inverse_trignometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asin();

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.5738766445872966, integral);
    }

    #[test]
    pub fn inverse_hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asinh();

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.6507421609432065, integral);
    }
}
