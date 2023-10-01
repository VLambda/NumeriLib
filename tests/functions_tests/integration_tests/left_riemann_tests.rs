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

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(55_f64, integral);
    }

    #[test]
    pub fn polynomial_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(2.25875, integral);
    }

    #[test]
    pub fn trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sin();

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.3695016461081666, integral);
    }

    #[test]
    pub fn logarithmic_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.ln();

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.36886153011820727, integral);
    }

    #[test]
    pub fn exponential_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.exp();

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(1.6756827432137449, integral);
    }

    #[test]
    pub fn inverse_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let intervals = 20_f64;
        let function = |x: f64| 1_f64 / x;

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.705803381792694, integral);
    }

    #[test]
    pub fn nonelementary_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.powf(x);

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.7846607642416729, integral);
    }

    #[test]
    pub fn hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.sinh();

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.5138137420591077, integral);
    }

    #[test]
    pub fn inverse_trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asin();

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.534606736417424, integral);
    }

    #[test]
    pub fn inverse_hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let intervals = 20_f64;
        let function = |x: f64| x.asinh();

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, intervals);

        assert_approx_eq!(0.44506465532975004, integral);
    }
}
