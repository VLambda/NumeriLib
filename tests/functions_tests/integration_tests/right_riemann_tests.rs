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

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(91_f64, integral);
    }

    #[test]
    pub fn polynomial_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(2.40875, integral);
    }

    #[test]
    pub fn trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sin();

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.4604313887907345, integral);
    }

    #[test]
    pub fn logarithmic_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.ln();

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.40351888914620454, integral);
    }

    #[test]
    pub fn exponential_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.exp();

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.7615968346366977, integral);
    }

    #[test]
    pub fn inverse_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| 1_f64 / x;

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.6808033817926941, integral);
    }

    #[test]
    pub fn nonelementary_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(x);

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.7846607642416729, integral);
    }

    #[test]
    pub fn hyperbolic() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sinh();

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.5725738017412977, integral);
    }

    #[test]
    pub fn inverse_trigonometric() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asin();

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.6131465527571689, integral);
    }

    #[test]
    pub fn inverse_hyperbolic() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asinh();

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.48913333468072717, integral);
    }
}
