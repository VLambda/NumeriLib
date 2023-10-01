use numerilib::Functions;
use std::f64::consts::FRAC_PI_4;

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn power_test() {
        let function = |x: f64| x.powf(2_f64);
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(4_f64, derivative, 1e-2)
    }

    #[test]
    pub fn polynomial_test() {
        let function = |x: f64| x.powf(2_f64) + 2_f64 * x + 1_f64;
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(6_f64, derivative, 1e-2)
    }

    #[test]
    pub fn trigonometric_test() {
        let function = |x: f64| x.sin();
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(-0.4161468365471424, derivative, 1e-2)
    }

    #[test]
    pub fn logarithmic_test() {
        let function = |x: f64| x.ln();
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(0.5, derivative, 1e-2)
    }

    #[test]
    pub fn exponential_test() {
        let function = |x: f64| x.exp();
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(7.38905609893065, derivative, 1e-2)
    }

    #[test]
    pub fn hyperbolic_test() {
        let function = |x: f64| x.sinh();
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(3.7621958703937253, derivative, 1e-2)
    }

    #[test]
    pub fn inverse_trigonometric_test() {
        let function = |x: f64| x.asin();
        let derivative = Functions::derivative(function, FRAC_PI_4);
        assert_approx_eq!(1.6155328208533604, derivative, 1e-2)
    }

    #[test]
    pub fn inverse_hyperbolic_test() {
        let function = |x: f64| x.asinh();
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(0.44721358616328644, derivative, 1e-2)
    }

    #[test]
    pub fn inverse_test() {
        let function = |x: f64| 1_f64 / x;
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(-0.25, derivative, 1e-2)
    }

    #[test]
    pub fn nonelementary_test() {
        let function = |x: f64| x.powf(x);
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(6.772589387082917, derivative, 1e-2)
    }
}
