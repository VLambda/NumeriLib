use numerilib::Functions;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn power_test() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let function = |x: f64| x.powf(2_f64);

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(72_f64, integral);
    }

    #[test]
    pub fn polynomial_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(2.3333333333333335, integral);
    }

    #[test]
    pub fn trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let function = |x: f64| x.sin();

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(1.416093124714195, integral);
    }

    #[test]
    pub fn logarithmic_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let function = |x: f64| x.ln();

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(0.38628789352450926, integral);
    }

    #[test]
    pub fn exponential_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let function = |x: f64| x.exp();

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(1.718281828459045, integral);
    }

    #[test]
    pub fn inverse_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let function = |x: f64| 1_f64 / x;

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(0.6931746031746032, integral);
    }

    #[test]
    pub fn nonelementary_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let function = |x: f64| x.powf(x);

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(0.7878041859552566, integral);
    }

    #[test]
    pub fn hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let function = |x: f64| x.sinh();

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(0.5430806536665995, integral);
    }

    #[test]
    pub fn inverse_trignometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let function = |x: f64| x.asin();

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(0.5833612698521974, integral);
    }

    #[test]
    pub fn inverse_hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let function = |x: f64| x.asinh();

        let integral = Functions::boole_rule(function, lower_bound, upper_bound);

        assert_approx_eq!(1.6514029494573559, integral);
    }
}
