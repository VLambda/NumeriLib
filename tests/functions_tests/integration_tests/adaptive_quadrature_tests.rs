use numerilib::Functions;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn power_test() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.powf(2_f64);

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(72_f64, integral);
    }

    #[test]
    pub fn polynomial_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(2.3333333333333335, integral);
    }

    #[test]
    pub fn trigonometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.sin();

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(1.416146836547128, integral);
    }

    #[test]
    pub fn logarithmic_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.ln();

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(0.3862943611198852, integral);
    }

    #[test]
    pub fn exponential_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.exp();

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(1.718281828459045, integral);
    }

    #[test]
    pub fn inverse_test() {
        let lower_bound = 1_f64;
        let upper_bound = 2_f64;
        let tolerence = 1e-20;
        let function = |x: f64| 1_f64 / x;

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(0.693147180559953, integral);
    }

    #[test]
    pub fn nonelementary_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.powf(x);

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(0.7834305107121501, integral);
    }

    #[test]
    pub fn hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.sinh();

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(0.5430806536665995, integral);
    }

    #[test]
    pub fn inverse_trignometric_test() {
        let lower_bound = 0_f64;
        let upper_bound = 1_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.asin();

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(0.5707963267949079, integral);
    }

    #[test]
    pub fn inverse_hyperbolic_test() {
        let lower_bound = 0_f64;
        let upper_bound = 2_f64;
        let tolerence = 1e-20;
        let function = |x: f64| x.asinh();

        let integral =
            Functions::adaptive_quadrature(function, lower_bound, upper_bound, tolerence);

        assert_approx_eq!(1.6512029728578204, integral);
    }
}
