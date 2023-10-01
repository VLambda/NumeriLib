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

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(71.5, integral);
    }

    #[test]
    pub fn polynomial_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(2.333125, integral);
    }

    #[test]
    pub fn trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sin();

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.4167370698755932, integral);
    }

    #[test]
    pub fn logarithmic_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.ln();

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.3863464311727383, integral);
    }

    #[test]
    pub fn exponential_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.exp();

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.7181028538189065, integral);
    }

    #[test]
    pub fn inverse_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| 1_f64 / x;

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.6930690982255869, integral);
    }

    #[test]
    pub fn nonelementary_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(x);

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.7828908607780994, integral);
    }

    #[test]
    pub fn hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sinh();

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(2.7610451118073858, integral);
    }

    #[test]
    pub fn inverse_trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asin();

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.5699366745646154, integral);
    }

    #[test]
    pub fn inverse_hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asinh();

        let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.6514334375596105, integral);
    }
}
